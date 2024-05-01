use anchor_lang::prelude::*;

use bellman::{
    circuits::num::Num,
    circuits::boolean::Boolean,
    circuits::{Circuit, ConstraintSystem},
    ff::{Field, PrimeField},
    group::Group,
    zk_snark::{Prover, Prover zk_snark::Prover::prepare, Verifier},
};

pub fn handler(address: &Pubkey) -> Pubkey {
    // Define the circuit
    struct EncryptAddressCircuit {
        address: Num<<bellman::ff::PrimeField as Field>::Repr>,
    }

    impl Circuit<bellman::ff::PrimeField> for EncryptAddressCircuit {
        fn synthesize(
            self,
            cs: &mut ConstraintSystem<<bellman::ff::PrimeField as Field>::Repr>,
        ) -> Result<(), bellman::SynthesisError> {
            // Define the address as a private input
            let address = cs.alloc_input(|| Ok(self.address), || "address")?;

            // Define the encrypted address as a public output
            let encrypted_address = cs.alloc(|| Ok(Num::zero()), || "encrypted_address")?;

            // Define the encryption circuit
            cs.enforce(|| "encryption",
                |lc| lc + address,
                |lc| lc + Num::from(0u8),
                |lc| lc + encrypted_address,
            );

            Ok(())
        }
    }

    // Prepare the prover and verifier
    let (pk, vk) = prepare::<EncryptAddressCircuit, _>(address);

    // Prove the encryption
    let proof = Prover::prove(pk, address).unwrap();

    // Verify the proof
    Verifier::verify(vk, proof).unwrap();

    // Return the encrypted address
    encrypted_address.to_pubkey()

}