<script lang="ts">
    import { getProfiles } from '$lib/api/profile-service';
</script>

{#await getProfiles()}
    <p class="loading"></p>
{:then profiles}
    {#each profiles as { id, username, display_name, avatar_url } (id)}
        <div>
            <img src={avatar_url} alt={`Avatar of ${username}`} />
            <h2>{display_name ?? `@${username}`}</h2>
        </div>
    {/each}
{/await}
