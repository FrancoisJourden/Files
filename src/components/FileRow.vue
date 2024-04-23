<script setup lang="ts">
import {DirectoryItem} from '../models/DirectoryItem.ts'
import {IconFileText} from "@tabler/icons-vue";

const props = defineProps({
  file: {required: true, type: Object as () => DirectoryItem}
})

</script>

<template>
  <div class="table-row">
    <div class="table-cell align-middle w-8">
      <IconFileText size="24" stroke-width="2" class="table-cell align-middle text-blue-600"/>
    </div>
    <div class="table-cell align-middle">
      <p class="truncate">{{ props.file.name }}</p>
    </div>
    <div class="table-cell align-middle">
      <p class="truncate">{{ props.file.size }}B</p>
    </div>
  </div>
</template>

<style scoped>

</style>