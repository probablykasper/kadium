<script lang="ts" context="module">
  import { writable } from 'svelte/store'

  export const modalCount = writable(0)
</script>

<script lang="ts">
  import { onDestroy, onMount } from 'svelte'
  import { scale } from 'svelte/transition'

  export let onCancel: () => void
  export let noEscapeHandling = false
  export let form: (() => void) | undefined = undefined
  $: tag = form === undefined ? 'div' : 'form'
  export let noCloseIcon = false
  export let title: string | null = null

  let dialogEl: HTMLDialogElement
  let backdrop = false

  $modalCount += 1
  onDestroy(() => {
    $modalCount -= 1
  })

  onMount(() => {
    dialogEl.showModal()
    setTimeout(() => {
      // wait for the dialog to be visible before animating. setTimeout requires for Safari
      backdrop = true
    })
    return () => {
      dialogEl.close()
    }
  })

  function pos_within(e: MouseEvent, el: HTMLElement) {
    const rect = el.getBoundingClientRect()
    return (
      rect.top <= e.clientY &&
      e.clientY <= rect.bottom &&
      rect.left <= e.clientX &&
      e.clientX <= rect.right
    )
  }

  // Prevent clicks where the mousedown or mouseup happened on a child element.
  let clickable = false
</script>

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<dialog
  class="modal"
  class:show-backdrop={backdrop}
  bind:this={dialogEl}
  on:mousedown|self={() => {
    clickable = true
  }}
  on:click|self={(e) => {
    if ((clickable || e.type !== 'click') && !pos_within(e, dialogEl)) {
      onCancel()
    }
  }}
  on:keydown
  on:keydown={(e) => {
    if (e.key === 'Escape' && noEscapeHandling) {
      e.preventDefault()
    }
  }}
  on:keydown|self={(e) => {
    if (form && e.key === 'Enter' && !e.metaKey) {
      form()
      e.preventDefault()
    }
  }}
  on:cancel={(e) => {
    e.preventDefault()
    onCancel()
  }}
  transition:scale={{ duration: 200, start: 0.93, opacity: 0 }}
  on:outrostart={() => {
    backdrop = false
  }}
>
  <svelte:element
    this={tag}
    class="box"
    on:submit|preventDefault={form}
    on:mousedown={() => {
      clickable = false
    }}
    role="none"
  >
    {#if !noCloseIcon}
      <svg
        role="none"
        on:click={() => onCancel()}
        fill="currentColor"
        xmlns="http://www.w3.org/2000/svg"
        width="12"
        height="12"
        viewBox="0 0 24 24"
        ><path
          d="M23.954 21.03l-9.184-9.095 9.092-9.174-2.832-2.807-9.09 9.179-9.176-9.088-2.81 2.81 9.186 9.105-9.095 9.184 2.81 2.81 9.112-9.192 9.18 9.1z"
        /></svg
      >
    {/if}
    {#if title !== null}
      <h2>{title}</h2>
    {/if}
    <slot />
    {#if $$slots.buttons}
      <div class="buttons">
        <slot name="buttons" />
      </div>
    {/if}
  </svelte:element>
</dialog>

<style lang="sass">
	dialog
		background-color: var(--modal-bg, hsl(220, 18%, 11%))
		color: inherit
		box-sizing: border-box
		border-radius: 8px
		box-shadow: 0px 0px 30px 0px rgba(#000000, 0.5)
		outline: none
		padding: 0px
		border: 1px solid hsla(0, 0%, 100%, 0.15)
	.box
		padding: 24px
	::backdrop
		transition: opacity 200ms cubic-bezier(0.33, 1, 0.68, 1) // easeOutCubic
		background-color: hsla(0, 0%, 0%, 0.2)
		opacity: 0
	.show-backdrop::backdrop
		opacity: 1
	svg
		position: absolute
		right: 12px
		top: 12px
		padding: 6px
		&:hover
			opacity: 0.7
	h2
		margin-top: -8px
	.buttons
		display: flex
		justify-content: flex-end
</style>
