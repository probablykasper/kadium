<script lang="ts">
  import Link from './Link.svelte'

  import type { Settings } from './data'

  export let settings: Settings
  console.log(settings)
</script>

{#each settings.groups as group}
  <h2 class="selectable">{group.name}</h2><Link>Edit</Link>
  <div class="text-container">
    <p class="selectable p">Minutes between refreshes: {group.minutes_between_refreshes}</p>
  </div>
  <div class="channels">
    {#each group.channels as channel}
      <div class="channel selectable">
        <img src={channel.icon} alt="" />
        <div class="details">
          <a href="https://youtube.com/channel/{channel.id}" target="_blank" class="title"
            >{channel.name}</a>
          <div class="content">
            <span>{channel.id}</span>
            <span>Check for videos after {new Date(channel.from_time).toLocaleString()}</span>
          </div>
        </div>
        <div class="spacer" />
        <Link>Edit</Link>
      </div>
    {/each}
  </div>
{/each}

<style lang="sass">
  .spacer
    flex-grow: 1
  h2
    display: inline-block
    margin-right: 5px
    margin-bottom: 0px
    font-weight: 600
  .selectable
    user-select: text
    -webkit-user-select: text
  .text-container
    margin-block-start: 1em
    margin-block-end: 1em
    p
      display: inline-block
      margin: 0px
  .channels
    display: flex
    flex-wrap: wrap
  .channel
    display: flex
    flex-grow: 1
    align-items: center
    border-radius: 7px
    transition: all 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
    padding: 15px 5px
    border: 1px solid transparent
    @media screen and (min-width: 800px)
      padding: 15px
      margin: 8px
      background-color: hsla(0, 0%, 100%, 0.03)
      box-shadow: 0px 4px 8px 0px hsla(0, 0%, 0%, 0.1)
      border: 1px solid hsla(0, 0%, 50%, 0.04)
      // border: 1px solid transparent
      &:hover
        border: 1px solid hsla(0, 0%, 50%, 0.2)
    img
      width: 50px
      max-width: 70px
      padding-right: 15px
    .details
      padding-right: 15px
      a.title
        color: inherit
        font-weight: bold
        font-size: 16px
        text-decoration: none
      .content
        font-size: 13px
        span
          display: block
          color: #808080
</style>
