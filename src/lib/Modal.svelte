<script lang="ts">
  export let visible = false
  let modalBg: HTMLDivElement
  let lastFocus: Element | null
  $: if (visible) {
    lastFocus = document.activeElement
  } else if (lastFocus instanceof HTMLElement) {
    lastFocus.focus()
  }
  $: if (visible && modalBg) {
    modalBg.focus()
    const firstInput = modalBg.querySelector('input, textarea')
    if (firstInput instanceof HTMLElement) {
      firstInput.focus()
    }
  }
  function close() {
    visible = false
  }
</script>

{#if visible}
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="modal-bg" on:click|self={close} tabindex="-1" on:keydown bind:this={modalBg}>
    <div class="box">
      <div class="spacer">
        <slot {close} />
      </div>
    </div>
  </div>
{/if}

<style lang="sass">
  .modal-bg
    position: fixed
    width: 100%
    height: 100%
    top: 0px
    left: 0px
    display: flex
    align-items: center
    justify-content: center
    z-index: 100
    padding: 20px
    box-sizing: border-box
    background-color: rgba(#000000, 0.5)
  .box
    background-color: hsl(220, 18%, 11%)
    border: 1px solid hsla(0, 0%, 100%, 0.1)
    min-width: 300px
    max-width: 100%
    max-height: 100%
    box-sizing: border-box
    border-radius: 7px
    box-shadow: 0px 0px 30px 0px rgba(#000000, 0.5)
    overflow: auto
  .spacer
    margin: 18px
    position: relative
</style>
