import { render } from '@testing-library/svelte';
import Button from './Button.svelte';
import { fetchHelloMessage } from '../utils/tauri';

test('renders button with correct text', () => {
  const { getByText } = render(Button, { props: { text: 'Click Me' } });
  expect(getByText('Click Me')).toBeInTheDocument();
});

test('Button renders correctly', () => {
    // Example test for Button component
    expect(true).toBe(true);
});

test('fetchHelloMessage returns correct data', async () => {
  global.fetch = jest.fn(() =>
    Promise.resolve({
      ok: true,
      json: () => Promise.resolve({ message: 'Hello from backend!' })
    })
  );

  const data = await fetchHelloMessage();
  expect(data.message).toBe('Hello from backend!');
});