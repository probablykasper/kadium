<script lang="ts">
  import { onMount, tick } from 'svelte'

  export let loaded = false

  type Options = {
    repeat?: boolean
    duration?: number
    delay?: number
    threshold?: number
    opacity?: number
    x?: number
    y?: number
    scale?: number
  }
  export let options: Options = {}
  $: baseStyle = options.duration == null ? '' : `transition-duration: ${options.duration}ms;`
  $: baseStyle += options.delay == null ? '' : `transition-delay: ${options.delay}ms;`

  $: style = baseStyle || null
  $: if (loaded) {
    style = baseStyle || null
  } else {
    if (options.opacity != null) {
      style += `opacity: ${options.opacity};`
    }

    let transform = ''
    if (options.x) transform += `translateX(${options.x}px)`
    if (options.y) transform += `translateY(${options.y}px)`
    if (options.scale) transform += `scale(${options.scale})`
    if (transform !== '') {
      style += `transform: ${transform};`
    }
  }

  let classes = ''
  export { classes as class }

  $: currentClass = classes + (loaded === true ? '' : ' ')

  let observer: IntersectionObserver
  let element: HTMLElement

  onMount(async () => {
    await tick()
    if (typeof IntersectionObserver === 'undefined') {
      return
    }
    observer = new IntersectionObserver((entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          loaded = true
        } else if (options.repeat) {
          loaded = false
        }
      })
    }, {})
    observer.observe(element)
    return () => observer.unobserve(element)
  })
</script>

<div bind:this={element} class={currentClass} style={style === null ? '' : style}>
  <slot {loaded} />
</div>
