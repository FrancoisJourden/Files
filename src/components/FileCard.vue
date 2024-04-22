<script setup lang="ts">
import {DirectoryItem} from '../models/DirectoryItem.ts'
import {IconFileFilled} from "@tabler/icons-vue";

const props = defineProps({
  file: {required: true, type: Object as () => DirectoryItem}
})
</script>

<template>
  <div class="text-center">
    <IconFileFilled size="80" stroke-width="2" class="m-auto text-blue-600"/>
    <p class="truncate">{{ props.file.name }}</p>
  </div>
</template>

<style scoped>

</style>