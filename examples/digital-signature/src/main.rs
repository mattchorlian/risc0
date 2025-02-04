// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::{Arg, Command};
use digital_signature::sign;
use sha2::{Digest, Sha256};

use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let matches = Command::new("sign")
        .version("0.1.0")
        .author("Risc0, Inc.")
        .about("Digital signatures with Risc0")
        .arg(Arg::new("message").required(true))
        .arg(
            Arg::new("passphrase")
                .long("passphrase")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let message = matches.value_of("message").unwrap();
    let passphrase = matches.value_of("passphrase").unwrap();
    let signing_receipt = sign(&passphrase, &message).unwrap();

    tracing::info!("Inputs");
    tracing::info!("\tmessage: {:?}", &message);
    tracing::info!("Commitment:");
    tracing::info!("\tmessage: {:?}", &signing_receipt.get_message().unwrap());
    tracing::info!("\tidentity: {:?}", &signing_receipt.get_identity().unwrap());
    tracing::info!("Integrity Checks:");
    let message_hash = &signing_receipt.get_message().unwrap().msg;
    let expected_message_hash = Sha256::digest(message);
    if message_hash != expected_message_hash.as_slice() {
        tracing::error!("Message commitment does not match given message!");
        std::process::exit(1);
    }
    tracing::info!("\tmessage: valid");
    if signing_receipt.verify().is_err() {
        tracing::error!("Receipt is invalid!");
        tracing::error!("{}", signing_receipt.verify().unwrap_err());
        std::process::exit(1);
    }
    tracing::info!("\treceipt: valid");
}

#[cfg(test)]
mod tests {
    use digital_signature::sign;
    use sha2::{Digest, Sha256};

    const MESSAGE: &str = "This is a signed message";
    const PASSPHRASE: &str = "passw0rd";

    #[test]
    fn main() {
        let signing_receipt = sign(&PASSPHRASE, &MESSAGE).unwrap();
        let message_hash = &signing_receipt.get_message().unwrap().msg;
        let expected_message_hash = Sha256::digest(MESSAGE);
        assert_eq!(
            message_hash,
            expected_message_hash.as_slice(),
            "Message commitment does not match given message!"
        );
        assert!(
            signing_receipt.verify().is_ok(),
            "Receipt is invalid! {}",
            signing_receipt.verify().unwrap_err()
        );
    }
}
