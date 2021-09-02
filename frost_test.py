from binascii import hexlify

import frost

params = frost.Parameters(3, 2)
alice = frost.Participant(params, 1)
bob = frost.Participant(params, 2)
carol = frost.Participant(params, 3)

# Verify ZKP
alice.verify_proof_of_secret_key()
bob.verify_proof_of_secret_key()
carol.verify_proof_of_secret_key()

alice_state = frost.DistributedKeyGenerationR1(params, alice, [bob, carol])
alice_their_secret_shares = alice_state.their_secret_shares()

bob_state = frost.DistributedKeyGenerationR1(params, bob, [alice, carol])
bob_their_secret_shares = bob_state.their_secret_shares()

carol_state = frost.DistributedKeyGenerationR1(params, carol, [alice, bob])
carol_their_secret_shares = carol_state.their_secret_shares()

# Advance to round 2
alice_my_secret_shares = [bytes(bob_their_secret_shares[0][1]), bytes(carol_their_secret_shares[0][1])]
alice_state = alice_state.to_round_two(1, alice_my_secret_shares)

bob_my_secret_shares = [bytes(alice_their_secret_shares[0][1]), bytes(carol_their_secret_shares[1][1])]
bob_state = bob_state.to_round_two(2, bob_my_secret_shares)

carol_my_secret_shares = [bytes(alice_their_secret_shares[1][1]), bytes(bob_their_secret_shares[1][1])]
carol_state = carol_state.to_round_two(3, carol_my_secret_shares)

alice_group_key, alice_secret_key, alice_public_key = alice_state.finish(alice)
bob_group_key, bob_secret_key, bob_public_key = bob_state.finish(bob)
carol_group_key, carol_secret_key, carol_public_key = carol_state.finish(carol)

assert alice_group_key[0] == bob_group_key[0], "Group keys should match!"
assert bob_group_key[0] == carol_group_key[0], "Group keys should match!"

print("Done generating distributed key, group key: %s" % hexlify(bytes(alice_group_key)).decode())

# ------- END KEY GEN ------- #

alice_public_comshares, alice_secret_comshares = frost.generate_commitment_share_lists(1, 1)
bob_public_comshares, bob_secret_comshares = frost.generate_commitment_share_lists(2, 1)
carol_public_comshares, carol_secret_comshares = frost.generate_commitment_share_lists(3, 1)

context = b"CONTEXT STRING STOLEN FROM DALEK TEST SUITE"
message = b"This is a test of the tsunami alert system. This is only a test."

msg_hash = bytes(frost.compute_message_hash(context, message))

# Signature aggregation
aggregator = frost.SignatureAggregatorInitial(params, bob_group_key, context, message)
aggregator.include_signer(1, alice_public_comshares[0], alice_public_key)
aggregator.include_signer(3, carol_public_comshares[0], carol_public_key)

signers = aggregator.get_signers()

alice_partial = frost.sign(1, alice_secret_key, msg_hash, alice_group_key, alice_secret_comshares, 0, signers)
carol_partial = frost.sign(3, carol_secret_key, msg_hash, carol_group_key, carol_secret_comshares, 0, signers)

alice_partial_hex = hexlify(bytes(alice_partial[1]))
carol_partial_hex = hexlify(bytes(carol_partial[1]))
print("Partial signature of Alice: %s" % alice_partial_hex.decode())
print("Partial signature of Carol: %s" % carol_partial_hex.decode())

aggregator.include_partial_signature(alice_partial)
aggregator.include_partial_signature(carol_partial)

aggregator = aggregator.finalize()

threshold_sig = aggregator.aggregate()
threshold_sig_hex = hexlify(bytes(threshold_sig[0] + threshold_sig[1]))
print("Resulting threshold Schnorr signature: %s" % threshold_sig_hex.decode())

if frost.verify(threshold_sig, alice_group_key, msg_hash) == 0:
    print("Signature invalid!")
else:
    print("Signature valid :)")
