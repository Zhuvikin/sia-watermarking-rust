import React from 'react';
import App from './App';
import { render } from '@testing-library/react';

test('renders learn react link', () => {
  let { getByText } = render(<App />);
  const linkElement = getByText(/Loading.../i);
  expect(linkElement).toBeInTheDocument();
});
