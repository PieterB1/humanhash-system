# LND Config
  mkdir -p /Volumes/PortableSSD/LND
  echo "datadir=/Volumes/PortableSSD/LND" > /Volumes/PortableSSD/LND/lnd.conf
  echo "" >> /Volumes/PortableSSD/LND/lnd.conf
  echo "[bitcoin]" >> /Volumes/PortableSSD/LND/lnd.conf
  echo "bitcoin.testnet=1" >> /Volumes/PortableSSD/LND/lnd.conf
  echo "bitcoin.node=neutrino" >> /Volumes/PortableSSD/LND/lnd.conf

  # Start LND
  lnd --lnddir=/Volumes/PortableSSD/LND

  # Unlock
  lncli --lnddir=/Volumes/PortableSSD/LND unlock

  # Get Info
  lncli --lnddir=/Volumes/PortableSSD/LND getinfo
  ```

