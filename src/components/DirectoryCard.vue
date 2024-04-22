<script setup lang="ts">
import { DirectoryItem } from '../models/DirectoryItem.ts'
import { IconFolderFilled } from "@tabler/icons-vue";

const props = defineProps({
  directory: {required: true, type: Object as () => DirectoryItem}
})
const emits = defineEmits<{
  (e: 'goToDir', path: string): void
}>()
</script>

<template>
  <div class="text-center" @dblclick="$emit('goToDir', props.directory?.path)">
    <IconFolderFilled size="80" stroke-width="2" class="m-auto text-blue-600"/>
    <p class="truncate">{{ props.directory.name }}</p>
  </div>
</template>

<style scoped>

</style>