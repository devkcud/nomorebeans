<script lang="ts">
    import Title from '$lib/components/Title.svelte';
    import IncomeExpenseChart from '$lib/components/IncomeExpenseChart.svelte';
    import { authStore } from '$lib/stores/auth.svelte';

    let currentProfile = $derived(authStore.currentProfile);

    const stats = [
        { label: 'Total Income', value: '$12,450.00', change: '+12.5%', positive: true },
        { label: 'Total Expenses', value: '$8,320.00', change: '+4.2%', positive: false },
        { label: 'Net Balance', value: '$4,130.00', change: '+8.3%', positive: true },
        { label: 'Active Projects', value: '7', change: '+2', positive: true }
    ];

    const recentTransactions = [
        { id: 1, client: 'Acme Corp', amount: 2500, date: '2025-11-20', type: 'income' },
        { id: 2, client: 'Software License', amount: -99, date: '2025-11-19', type: 'expense' },
        { id: 3, client: 'TechStart Inc', amount: 3200, date: '2025-11-18', type: 'income' },
        { id: 4, client: 'Office Supplies', amount: -145, date: '2025-11-17', type: 'expense' },
        { id: 5, client: 'WebDev LLC', amount: 1800, date: '2025-11-16', type: 'income' }
    ];
</script>

<div class="relative space-y-6 p-8">
    <header class="flex items-center justify-between">
        <div>
            <Title header={1} size="4xl" colored bold>
                Welcome back, {currentProfile?.displayName || currentProfile?.username}!
            </Title>
            <p class="mt-2 text-sm text-base-content/60">Here's your financial overview</p>
        </div>
    </header>

    <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-4">
        {#each stats as stat}
            <div class="card bg-base-200 p-6">
                <p class="text-sm text-base-content/60">{stat.label}</p>
                <p class="mt-2 text-3xl font-bold">{stat.value}</p>
                <p class="mt-2 text-sm {stat.positive ? 'text-success' : 'text-error'}">
                    {stat.change} from last month
                </p>
            </div>
        {/each}
    </div>

    <div class="grid grid-cols-1 gap-6 lg:grid-cols-3">
        <div class="lg:col-span-2">
            <div class="card bg-base-200 p-6">
                <h2 class="text-xl font-semibold mb-4">Income vs Expenses</h2>
                <div class="h-64">
                    <IncomeExpenseChart />
                </div>
            </div>
        </div>

        <div class="card bg-base-200 p-6">
            <h2 class="text-xl font-semibold mb-4">Quick Actions</h2>
            <div class="space-y-3">
                <button class="btn btn-primary w-full">Add Income</button>
                <button class="btn btn-secondary w-full">Add Expense</button>
                <button class="btn btn-outline w-full">New Invoice</button>
                <button class="btn btn-outline w-full">View Reports</button>
            </div>
        </div>
    </div>

    <div class="card bg-base-200 p-6">
        <h2 class="text-xl font-semibold mb-4">Recent Transactions</h2>
        <div class="overflow-x-auto">
            <table class="table w-full">
                <thead>
                    <tr>
                        <th>Date</th>
                        <th>Description</th>
                        <th>Type</th>
                        <th class="text-right">Amount</th>
                    </tr>
                </thead>
                <tbody>
                    {#each recentTransactions as transaction}
                        <tr class="hover">
                            <td>{transaction.date}</td>
                            <td>{transaction.client}</td>
                            <td>
                                <span class="badge {transaction.type === 'income' ? 'badge-success' : 'badge-error'}">
                                    {transaction.type}
                                </span>
                            </td>
                            <td class="text-right font-semibold {transaction.amount > 0 ? 'text-success' : 'text-error'}">
                                ${Math.abs(transaction.amount).toFixed(2)}
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    </div>
</div>
