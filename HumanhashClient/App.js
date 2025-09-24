import React, { useState } from 'react';
import { StyleSheet, Text, View, TextInput, TouchableOpacity, Alert } from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';

export default function App() {
  const [humanHashId, setHumanHashId] = useState('');
  const [proof, setProof] = useState('');

  const handleKycRequest = async () => {
    try {
      const response = await fetch('http://localhost:3003/oracle/kyc', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ human_hash_id: humanHashId }),
      });
      const data = await response.json();
      Alert.alert('KYC Response', JSON.stringify(data));
    } catch (error) {
      Alert.alert('Error', error.message);
    }
  };

  const handleZkpRequest = async () => {
    try {
      const response = await fetch('http://localhost:3003/oracle/zkp', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ human_hash_id: humanHashId, proof }),
      });
      const data = await response.json();
      Alert.alert('ZKP Response', JSON.stringify(data));
    } catch (error) {
      Alert.alert('Error', error.message);
    }
  };

  return (
    <LinearGradient colors={['#FFFFFF', '#E6F0FA']} style={styles.container}>
      <Text style={styles.title}>HumanHash Client</Text>
      <TextInput
        style={styles.input}
        placeholder="Human Hash ID"
        value={humanHashId}
        onChangeText={setHumanHashId}
      />
      <TextInput
        style={styles.input}
        placeholder="Proof"
        value={proof}
        onChangeText={setProof}
      />
      <TouchableOpacity style={styles.button} onPress={handleKycRequest}>
        <Text style={styles.buttonText}>Request KYC</Text>
      </TouchableOpacity>
      <TouchableOpacity style={styles.button} onPress={handleZkpRequest}>
        <Text style={styles.buttonText}>Request ZKP</Text>
      </TouchableOpacity>
    </LinearGradient>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20,
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#333',
    marginBottom: 20,
  },
  input: {
    width: '80%',
    padding: 15,
    marginVertical: 10,
    borderRadius: 25,
    backgroundColor: '#FFF',
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 8,
    elevation: 5,
  },
  button: {
    width: '80%',
    padding: 15,
    marginVertical: 10,
    borderRadius: 25,
    backgroundColor: '#007AFF',
    alignItems: 'center',
  },
  buttonText: {
    color: '#FFF',
    fontSize: 16,
    fontWeight: '600',
  },
});
