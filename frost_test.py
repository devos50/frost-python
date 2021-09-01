import frost

params = frost.Parameters(3, 2)
alice = frost.Participant(params, 1)
bob = frost.Participant(params, 2)
carol = frost.Participant(params, 3)

# Verify ZKP
alice.verify_proof_of_secret_key()
bob.verify_proof_of_secret_key()
carol.verify_proof_of_secret_key()

print(alice.coefficients)