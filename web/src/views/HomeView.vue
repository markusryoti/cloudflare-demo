<script setup lang="ts">
import { useQuery } from '@tanstack/vue-query'

interface Todo {
  id: string
  title: string
  completed: boolean
}

const { data, isLoading, error } = useQuery({
  queryKey: ['todos'],
  queryFn: () =>
    fetch('http://localhost:8787/todos')
      .then((res) => res.json() as Promise<{ todos: Todo[] }>)
      .then((data) => data.todos),
})
</script>

<template>
  <main>
    <div v-if="isLoading">Loading...</div>
    <div v-else-if="error">An error occurred: {{ error.message }}</div>
    <pre v-else>{{ data }}</pre>
  </main>
</template>
