<script lang="ts">
  import type { Settings } from './data'
  export let settings: Settings
  console.log(settings)
</script>

{#each settings.groups as group}
  <h2>{group.email}</h2>
  <p>Minutes between refreshes: {group.minutes_between_refreshes}</p>
  <div class="channels">
    {#each group.channels as channel}
      <div class="channel">
        <img class="col" src={channel.icon} alt="" />
        <div class="col details">
          <span class="title">{channel.name}</span>
          <div class="content">
            <span>{channel.id}</span>
            <span>Check for videos after {new Date(channel.from_time).toLocaleString()}</span>
          </div>
        </div>
      </div>
    {/each}
  </div>
{/each}

<style lang="sass">
  .channels
    display: flex
    flex-wrap: wrap
    justify-content: center
    padding: 20px
  .channel
    display: flex
    align-items: center
    .col
      padding: 5px 10px
    img
      width: 50px
    .details
      .title
        font-weight: bold
        font-size: 16px
      span
        display: block
      .content
        font-size: 14px
        span
          opacity: 0.5
</style>
