import React, { useState } from 'react';
import axios from 'axios';
import { encryptBiometric } from '../utils/crypto.ts';


interface Props {
  token: string;
  userId: number;
  onResult: (result: string) => void;
}

const BiometricUpload: React.FC<Props> = ({ token, userId, onResult }) => {
  const [file, setFile] = useState<File | null>(null);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) setFile(e.target.files[0]);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!file || !token) {
      onResult('No file or token provided');
      return;
    }

    // Temporarily disable FaceTecSDK calls
    /*
    await FaceTecSDK.initialize();
    const livenessResult = await FaceTecSDK.performLivenessCheck(file);
    if (!livenessResult.isLive) {
      onResult('Liveness check failed');
      return;
    }
    */

    const formData = new FormData();
    const encryptedData = encryptBiometric(file.name, 'vault-key');
    formData.append('biometric', file, encryptedData);
    // formData.append('livenessData', JSON.stringify(livenessResult));

    try {
      const response = await axios.post('http://system-api:3000/api/biometric/verify', formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
          Authorization:'Bearer ${token}' ,
       },
      });
      onResult(response.data.message);
    } catch (err: any) {
      onResult('Verification failed: ' + err.response?.data.message);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <input type="file" accept="image/*" onChange={handleFileChange} />
      <button type="submit">Verify Biometric</button>
    </form>
  );
};

export default BiometricUpload;
