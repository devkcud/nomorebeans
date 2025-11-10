<script lang="ts">
    import { getProfiles } from '$lib/api/profile-service';
</script>

{#await getProfiles()}
    <p class="loading"></p>
{:then profiles}
    {#each profiles as { username, display_name, avatar_url } (username)}
        <div class="">
            <img class="" src={avatar_url} alt={`Avatar of ${username}`} />
            <h2 class="">{display_name ?? `@${username}`}</h2>
        </div>
    {/each}
{/await}
