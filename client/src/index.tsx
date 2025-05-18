import React from 'react';
import ReactDOM from 'react-dom/client';
import BiometricUpload from './components/BiometricUpload.tsx';
import BiometricUpload from './components/BiometricUpload';

const root = ReactDOM.createRoot(document.getElementById('root')!);
root.render(
  <React.StrictMode>
    <BiometricUpload token="test-token" userId={1} onResult={console.log} />
  </React.StrictMode>
);
