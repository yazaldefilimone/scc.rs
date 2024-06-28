<samp>

```ascii
| .............................. process ............................... |
| ....... parse ....    |   ..... run ......  |   .... render .......... |

              +--------+                          +----------+
Input    ->-  | Parser |     ->-    AST    ->-    | Compiler |   ->-  JSX/Vue/HTML...
              +--------+             |            +----------+
                                     X
                                     |
                              +--------------+
                              | Transformers |
                              +--------------+

```

- How SCC works?

1. Markdown(with jsx/tsx)

```mdx
export function Thing() {
  return <>World</>;
}

# Hello <Thing />
```

- Result

```jsx
/* @jsxRuntime automatic */
/* @jsxImportSource react */

export function Thing() {
  return <>World</>;
}

export default function MDXContent() {
  return (
    <h1>
      Hello <Thing />
    </h1>
  );
}
```

2. Markdown(with vue)

```html
<script setup>
  import {ref} from 'vue' const count = ref(0)
</script>

## Markdown Content 

The count is: {{ count }}

<button @click="count++">Increment</button>
```

- Result

```vue
<script setup>
import { ref } from "vue";

const count = ref(0);
</script>

<template>
  <h1>Markdown Content</h1>
  <p>The count is: {{ count }}</p>
  <button @click="count++">Increment</button>
</template>
```
