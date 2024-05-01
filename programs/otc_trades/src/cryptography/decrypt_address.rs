use anchor_lang::prelude::*;

use bellman::{
    circuits::{boolean::Boolean, num::Num, Circuit, ConstraintSystem},
    ff::{Field, PrimeField},
    group::Group,
    zk_snark::{zk_snark::Prover::prepare, Verifier},
};

fn handler(encrypted_address: &Pubkey) -> Pubkey {
    // Define the circuit
    struct DecryptAddressCircuit {
        encrypted_address: Num<<bellman::ff::PrimeField as Field>::Repr>,
    }

    impl Circuit<bellman::ff::PrimeField> for DecryptAddressCircuit {
        fn synthesize(
            self,
            cs: &mut ConstraintSystem<<bellman::ff::PrimeField as Field>::Repr>,
        ) -> Result<(), bellman::SynthesisError> {
            // Define the encrypted address as a private input
            let encrypted_address = cs.alloc_input(|| Ok(self.encrypted_address), || "encrypted_address")?;

            // Define the decrypted address as a public output
            let decrypted_address = cs.alloc(|| Ok(Num::zero()), || "decrypted_address")?;

            // Define the decryption circuit
            cs.enforce(|| "decryption",
                |lc| lc + encrypted_address,
                |lc| lc - Num::from(0u8),
                |lc| lc + decrypted_address,
            );

            Ok(())
        }
    }

    // Prepare the prover and verifier
    let (pk, vk) = prepare::<DecryptAddressCircuit, _>(encrypted_address);

    // Prove the decryption
    let proof = Prover::prove(pk, encrypted_address).unwrap();

    // Verify the proof
    Verifier::verify(vk, proof).unwrap();

    // Return the decrypted address
    decrypted_address.to_pubkey()
}