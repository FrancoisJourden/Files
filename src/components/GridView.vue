<script setup lang="ts">

import FileCard from "./FileCard.vue";
import DirectoryCard from "./DirectoryCard.vue";
import {DirectoryItem} from "../models/DirectoryItem.ts";
import {computed} from "vue";

const props = defineProps<{
  files: Array<DirectoryItem>
}>();

const files = computed(() => [...props.files].sort((a, b) => {
  if (a.is_dir && !b.is_dir) return -1;
  if (!a.is_dir && b.is_dir) return 1;
  return a.name.localeCompare(b.name)
}))

defineEmits<{
  "setDir": [path: string]
}>()

</script>

<template>
  <div class="w-full grid gap-2 grid-template-100 px-1">
    <div v-for="item in files">
      <DirectoryCard v-if="item.is_dir" :directory="item" @set-dir="path => $emit('setDir', path)"/>
      <FileCard v-else :file="item"/>
    </div>
  </div>
</template>

<style scoped>
.grid-template-100 {
  grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
}
</style>