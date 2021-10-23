<script lang="ts">
  import type { Settings } from './data'
  export let settings: Settings
  console.log(settings)
</script>

<div class="page">
  <button class="normal">Setup</button>

  {#each settings.groups as group}
    <h2>{group.email} <button class="text">Edit</button></h2>
    <p>Minutes between refreshes: {group.minutes_between_refreshes}</p>
    <div class="channels">
      {#each group.channels as channel}
        <div class="channel">
          <img src={channel.icon} alt="" />
          <div class="details">
            <a href="https://youtube.com/channel/{channel.id}" target="_blank" class="title"
              >{channel.name}</a>
            <div class="content">
              <span>{channel.id}</span>
              <span>Check for videos after {new Date(channel.from_time).toLocaleString()}</span>
            </div>
          </div>
          <button class="text space-left">Edit</button>
        </div>
      {/each}
    </div>
  {/each}
</div>

<style lang="sass">
  .page
    padding: 20px
  .space-left
    margin-left: auto
  h2
    font-weight: 600
  button.text
    cursor: pointer
    font-size: 13px
    vertical-align: middle
    color: hsl(210, 100%, 55%)
    background-color: transparent
    padding: 0px
    border: none
    &:hover
      color: hsl(210, 100%, 45%)
  button.normal
    cursor: pointer
    padding: 9px 22px
    font-size: 13px
    border: none
    color: #ffffff
    font-weight: 500
    border-radius: 8px
    background-color: transparent
    position: relative
    outline: none
    transition: all 180ms cubic-bezier(0.4, 0.0, 0.2, 1)
    &::before
      content: ''
      position: absolute
      z-index: -1
      left: 0px
      top: 0px
      bottom: 0px
      right: 0px
      border-radius: 10px
      background-color: hsl(220, 100%, 46%)
      border: 1px solid hsla(0, 0%, 100%, 0.1)
      transition: all 80ms cubic-bezier(0.4, 0.0, 0.2, 1)
    &:hover::before
      background-color: hsl(220, 100%, 52%)
      transform: scale(1.04, 1)
    &:focus::before
      box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
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
