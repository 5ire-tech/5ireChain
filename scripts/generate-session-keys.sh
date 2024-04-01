
# Mnemonic Phrase Test
# prefer butter fluid ivory eyebrow pony gym snap divert wide run foam


if [ -z "$MNEMONIC" ]; then
  MNEMONIC=`subkey generate --output-type Json | jq -r '.secretPhrase'`
fi

KEY=`subkey inspect "$MNEMONIC" --output-type Json`
SEED=`jq -r '.secretSeed' <<< $KEY `
SR25519_SS58=`subkey inspect --scheme sr25519 "$MNEMONIC" --output-type Json 2>&1 | jq -r '.ss58PublicKey'`
ED25519_SS58=`subkey inspect --scheme ed25519 "$MNEMONIC" --output-type Json 2>&1 | jq -r '.ss58PublicKey'`
SR25519_PUB=`subkey inspect --scheme sr25519 "$MNEMONIC" --output-type Json 2>&1 | jq -r '.publicKey'`
ED25519_PUB=`subkey inspect --scheme ed25519 "$MNEMONIC" --output-type Json 2>&1 | jq -r '.publicKey'`


echo "****************** account data ******************"
echo "secret_seed:      $SEED"
echo "mnemonic:         $MNEMONIC"
echo "sr25519 address:  $SR25519_PUB (SS58: $SR25519_SS58)"
echo "ed25519 address:  $ED25519_PUB (SS58: $ED25519_SS58)"
echo "    [
    \"$SR25519_SS58\",
    \"$ED25519_SS58\",
    {
        \"grandpa\": \"$ED25519_SS58\",
        \"babe\": \"$SR25519_SS58\",
        \"im_online\": \"$SR25519_SS58\",
        \"authority_discovery\": \"$SR25519_SS58\",
    }
]"

\