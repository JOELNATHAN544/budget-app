import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { Pie } from 'react-chartjs-2';
import { Chart, ArcElement, Tooltip, Legend } from 'chart.js';

Chart.register(ArcElement, Tooltip, Legend);

interface Transaction {
  id: number;
  amount: number;
  category: string;
  description?: string;
  date: string;
}

function App() {
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [amount, setAmount] = useState('');
  const [category, setCategory] = useState('Food');
  const [description, setDescription] = useState('');
  const [editingId, setEditingId] = useState<number | null>(null);
  const [editAmount, setEditAmount] = useState('');
  const [editCategory, setEditCategory] = useState('Food');
  const [total, setTotal] = useState(0);

  // Fetch transactions
  useEffect(() => {
    axios.get<Transaction[]>('http://localhost:8000/api/transactions')
      .then(res => {
        const transactionsData = res.data;
        const processedTransactions = [];
        for (const t of transactionsData) {
          processedTransactions.push({
            ...t,
            amount: parseFloat(t.amount as any)
          });
        }
        setTransactions(processedTransactions);
        updateTotal(processedTransactions);
      });
  }, []);

  const updateTotal = (transactions: Transaction[]) => {
    setTotal(transactions.reduce((sum, t) => sum + t.amount, 0));
  };

  // Add transaction
  const addTransaction = () => {
    const newTransaction = {
      amount: amount.toString(),
      category,
      description,
      date: new Date().toISOString()
    };

    axios.post<Transaction>('http://localhost:8000/api/transactions', newTransaction)
      .then(res => {
        const addedTransaction = { ...res.data, amount: parseFloat(res.data.amount as any) };
        const updated = [...transactions, addedTransaction];
        setTransactions(updated);
        updateTotal(updated);
        setAmount('');
        setDescription('');
      });
  };

  // Delete transaction
  const deleteTransaction = (id: number) => {
    axios.delete(`http://localhost:8000/api/transactions/${id}`)
      .then(() => {
        const updated = transactions.filter(t => t.id !== id);
        setTransactions(updated);
        updateTotal(updated);
      });
  };

  // Edit transaction
  const saveEdit = () => {
    if (!editingId) return;

    axios.put(`http://localhost:8000/api/transactions/${editingId}`, {
      amount: parseFloat(editAmount),
      category: editCategory
    }).then(res => {
      const updated = transactions.map(t => 
        t.id === editingId ? res.data : t
      );
      setTransactions(updated);
      updateTotal(updated);
      setEditingId(null);
    });
  };

  // Chart data
  const chartData = {
    labels: ['Food', 'Rent', 'Entertainment'],
    datasets: [{
      data: [
        transactions.filter(t => t.category === 'Food').reduce((sum, t) => sum + t.amount, 0),
        transactions.filter(t => t.category === 'Rent').reduce((sum, t) => sum + t.amount, 0),
        transactions.filter(t => t.category === 'Entertainment').reduce((sum, t) => sum + t.amount, 0)
      ],
      backgroundColor: ['#FF6384', '#36A2EB', '#FFCE56']
    }]
  };

  return (
    <div className="container">
      <h1>Budget Tracker</h1>
      <div className="form">
        <input
          type="number"
          placeholder="Amount"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
        />
        <select
          value={category}
          onChange={(e) => setCategory(e.target.value)}
        >
          <option value="Food">Food</option>
          <option value="Rent">Rent</option>
          <option value="Entertainment">Entertainment</option>
        </select>
        <input
          placeholder="Description"
          value={description}
          onChange={(e) => setDescription(e.target.value)}
        />
        <button onClick={addTransaction}>Add</button>
      </div>

      <div className="chart">
        <Pie data={chartData} />
        <h3>Total: ${total.toFixed(2)}</h3>
      </div>

      <div className="transactions">
        <h2>Transactions</h2>
        <ul>
          {transactions.map(t => (
            <li key={t.id}>
              {editingId === t.id ? (
                <>
                  <input
                    type="number"
                    value={editAmount}
                    onChange={(e) => setEditAmount(e.target.value)}
                  />
                  <select
                    value={editCategory}
                    onChange={(e) => setEditCategory(e.target.value)}
                  >
                    <option value="Food">Food</option>
                    <option value="Rent">Rent</option>
                  </select>
                  <button onClick={saveEdit}>Save</button>
                </>
              ) : (
                <>
                  <span>${t.amount.toFixed(2)} - {t.category}</span>
                  {t.description && <em>({t.description})</em>}
                  <button onClick={() => {
                    setEditingId(t.id);
                    setEditAmount(t.amount.toString());
                    setEditCategory(t.category);
                  }}>Edit</button>
                  <button onClick={() => deleteTransaction(t.id)}>Delete</button>
                </>
              )}  
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}

export default App;