import { render, waitFor } from '@testing-library/svelte';
import Dashboard from '../routes/dashboard.svelte';

test('Dashboard renders and shows statistik', async () => {
  const { getByText } = render(Dashboard);
  await waitFor(() => {
    expect(getByText('Dashboard Trading')).toBeInTheDocument();
    expect(getByText('Statistik')).toBeInTheDocument();
  });
});
