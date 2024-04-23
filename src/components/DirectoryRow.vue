<script setup lang="ts">
import { DirectoryItem } from '../models/DirectoryItem.ts'
import { IconFolderFilled } from "@tabler/icons-vue";

const props = defineProps({
  directory: {required: true, type: Object as () => DirectoryItem}
})

defineEmits<{
  (e: 'setDir', path: string): void
}>()

</script>

<template>
  <div class="table-row cursor-pointer" @dblclick="$emit('setDir', props.directory?.path)">
    <div class="table-cell align-middle w-8">
      <IconFolderFilled size="24" stroke-width="2" class="text-blue-600"/>
    </div>
    <div class="table-cell align-middle">
      <p class="truncate">{{ props.directory.name }}</p>
    </div>
    <div class="table-cell align-middle">
      <p v-if="props.directory?.size == 0">Empty</p>
      <p v-else-if="props.directory?.size == 1">1 item</p>
      <p v-else>{{ props.directory?.size }} items</p>
    </div>
  </div>
</template>

<style scoped>

</style>