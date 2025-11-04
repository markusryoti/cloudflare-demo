<script setup lang="ts">
fetch('http://localhost:8787/todos')
  .then((res) => res.json())
  .then((data) => console.log(data))
</script>

<template>
  <main>Hey</main>
</template>
