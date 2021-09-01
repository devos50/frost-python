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

# ------- END KEY GEN ------- #
