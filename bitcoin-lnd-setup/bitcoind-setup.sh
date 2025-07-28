# Start Bitcoind
  bitcoind -daemon -testnet -datadir=/Volumes/T9/bitcoin-data -walletdir=/Volumes/T9/bitcoin-data/wallets -txindex=1 -server=1 -rpcbind=0.0.0.0 -rpcallowip=0.0.0.0/0 -rpcuser=mybitcoinrpcuser -rpcpassword=X7pQz9kW3mN8vT2rY6jL -zmqpubrawblock=tcp://0.0.0.0:28332 -zmqpubrawtx=tcp://0.0.0.0:28333 -dbcache=4000 -rpcworkqueue=32 -debug=1

  # Create Wallet
  bitcoin-cli -rpcuser=mybitcoinrpcuser -rpcpassword=X7pQz9kW3mN8vT2rY6jL -rpcport=18332 createwallet testwallet true

  # Import Descriptors
  bitcoin-cli -rpcuser=mybitcoinrpcuser -rpcpassword=X7pQz9kW3mN8vT2rY6jL -rpcport=18332 -rpcwallet=testwallet importdescriptors '[{"desc": "wpkh(tpubD6NzVbkrYhZ4YZgk5ZVvFbjS2T7V2up3zEkbCedxCAYkQw55iRRFtUYnfca5oMhV17L3XXY5b6SLKMZBBP2scE9bK1rncM1hVG8heMNXGPt/0/*)#46zlu9xv", "timestamp": 0, "internal": false}, {"desc": "wpkh(tpubD6NzVbkrYhZ4YZgk5ZVvFbjS2T7V2up3zEkbCedxCAYkQw55iRRFtUYnfca5oMhV17L3XXY5b6SLKMZBBP2scE9bK1rncM1hVG8heMNXGPt/1/*)#yw87psk5", "timestamp": 0, "internal": true}]'

  # Derive Address
  bitcoin-cli -rpcuser=mybitcoinrpcuser -rpcpassword=X7pQz9kW3mN8vT2rY6jL -rpcport=18332 -rpcwallet=testwallet deriveaddresses "wpkh(tpubD6NzVbkrYhZ4YZgk5ZVvFbjS2T7V2up3zEkbCedxCAYkQw55iRRFtUYnfca5oMhV17L3XXY5b6SLKMZBBP2scE9bK1rncM1hVG8heMNXGPt/0/*)#46zlu9xv" [0,0]

  # Check Balance
  bitcoin-cli -rpcuser=mybitcoinrpcuser -rpcpassword=X7pQz9kW3mN8vT2rY6jL -rpcport=18332 -rpcwallet=testwallet getbalance
  ```

