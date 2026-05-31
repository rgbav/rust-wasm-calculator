import { useState } from 'react';
import './Calculator.css';
import { evaluate_expression } from 'calculator-wasm';

const BUTTONS = [
  '(', ')', 'C', '/',
  '7', '8', '9', '*',
  '4', '5', '6', '-',
  '1', '2', '3', '+',
  '0', '.', '=',
];

export default function Calculator() {
  const [expr, setExpr] = useState('');
  const [result, setResult] = useState('');
  const [isError, setIsError] = useState(false);

  const handleClick = (label: string) => {
    if (label === 'C') {
      setExpr('');
      setResult('');
      setIsError(false);
      return;
    }
    if (label === '=') {
      try {
        // call into wasm module
        const value = evaluate_expression(expr);
        setResult(String(value));
        setIsError(false);
      } catch (err) {
        console.error(err);
        setResult('Error');
        setIsError(true);
      }
      return;
    }
    setExpr((prev) => prev + label);
  };

  return (
    <div className="calculator">
      <h1>Calculator</h1>
      <div className="display">
        <div className="expr">{expr || '0'}</div>
        <div className={isError ? 'result error' : 'result'}>{result}</div>
      </div>
      <div className="keys">
        {BUTTONS.map((label) => (
          <button
            key={label}
            className={label === '=' ? 'key equals' : 'key'}
            onClick={() => handleClick(label)}
          >
            {label}
          </button>
        ))}
      </div>
    </div>
  );
}
