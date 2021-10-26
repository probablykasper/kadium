<button on:click on:mousedown><slot /></button>

<style lang="sass">
  button
    cursor: pointer
    padding: 8px 20px
    margin: 2px
    font-size: 14px
    border: none
    color: #ffffff
    font-weight: 500
    border-radius: 8px
    background-color: transparent
    position: relative
    outline: none
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
      transition: all 120ms cubic-bezier(0.4, 0.0, 0.2, 1)
    &:hover::before
      background-color: hsl(220, 100%, 52%)
      transform: scale(1.04, 1)
    &:focus::before
      box-shadow: 0px 0px 0px 3px hsla(220, 100%, 50%, 0.5)
</style>
