# LND Config
mkdir -p /Users/pieterwjbouwer/Library/Application\ Support/Lnd
echo "datadir=/Users/pieterwjbouwer/Library/Application Support/Lnd" > /Users/pieterwjbouwer/Library/Application\ Support/Lnd/lnd.conf
echo "" >> /Users/pieterwjbouwer/Library/Application\ Support/Lnd/lnd.conf
echo "[Bitcoin]" >> /Users/pieterwjbouwer/Library/Application\ Support/Lnd/lnd.conf
echo "bitcoin.active=1" >> /Users/pieterwjbouwer/Library/Application\ Support/Lnd/lnd.conf
echo "bitcoin.testnet=1" >> /Users/pieterwjbouwer/Library/Application\ Support/Lnd/lnd.conf
echo "bitcoin.node=bitcoind" >> /Users/pieterwjbouwer/Library/Application\ Support/Lnd/lnd.conf
echo "bitcoind.rpchost=localhost:18332" >> /Users/pieterwjbouwer/Library/Application\ Support/Lnd/lnd.conf
echo "bitcoind.rpcuser=mybitcoinrpcuser" >> /Users/pieterwjbouwer/Library/Application\ Support/Lnd/lnd.conf
echo "bitcoind.rpcpass=X7pQz9kW3mN8vT2rY6jL" >> /Users/pieterwjbouwer/Library/Application\ Support/Lnd/lnd.conf
echo "bitcoind.zmqpubrawblock=tcp://127.0.0.1:28332" >> /Users/pieterwjbouwer/Library/Application\ Support/Lnd/lnd.conf
echo "bitcoind.zmqpubrawtx=tcp://127.0.0.1:28333" >> /Users/pieterwjbouwer/Library/Application\ Support/Lnd/lnd.conf

# Start LND
lnd --lnddir=/Users/pieterwjbouwer/Library/Application\ Support/Lnd

# Unlock
lncli --lnddir=/Users/pieterwjbouwer/Library/Application\ Support/Lnd unlock

# Get Info
lncli --lnddir=/Users/pieterwjbouwer/Library/Application\ Support/Lnd getinfo
