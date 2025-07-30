import { useState } from 'react';

export default function App() {
  const [guess, setGuess] = useState('');
  const [message, setMessage] = useState('');
  const [gameOver, setGameOver] = useState(false);

  const handleGuess = async () => {
    try {
      const res = await fetch(`${import.meta.env.VITE_API_URL}/api/guess`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ guess: Number(guess) }),
      });

      if (!res.ok) {
        throw new Error('Network response was not ok');
      }

      const data = await res.json();
      setMessage(data.message);

      if (data.status === 'win') {
        setGameOver(true);
      }
    } catch (error) {
      console.error('Error:', error);
      setMessage('Something went wrong. Please try again.');
    }
  };

  const resetGame = () => {
    setGuess('');
    setMessage('');
    setGameOver(false);
  };

  return (
    <div className="flex flex-col items-center justify-center h-screen gap-4 bg-gray-100">
      <h1 className="text-3xl font-bold">Guess the Number</h1>

      {!gameOver ? (
        <>
          <input
            type="number"
            value={guess}
            onChange={(e) => setGuess(e.target.value)}
            className="p-2 border rounded"
            placeholder="Enter your guess"
          />
          <button
            onClick={handleGuess}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
          >
            Submit Guess
          </button>
        </>
      ) : (
        <button
          onClick={resetGame}
          className="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
        >
          Play Again
        </button>
      )}

      {message && <p className="text-lg mt-4">{message}</p>}
    </div>
  );
}
