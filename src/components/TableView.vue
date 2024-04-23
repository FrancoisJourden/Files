<script setup lang="ts">

import {DirectoryItem} from "../models/DirectoryItem.ts";
import DirectoryRow from "./DirectoryRow.vue";
import FileRow from "./FileRow.vue";
import {computed, Ref, ref} from "vue";
import {IconCaretUpFilled, IconCaretDownFilled} from "@tabler/icons-vue";

const props = defineProps<{
  files: Array<DirectoryItem>
}>();

defineEmits<{
  "setDir": [path: string]
}>()

const sortBy: Ref<"name" | "size"> = ref("name");
const sortOrder: Ref<"asc" | "desc"> = ref("asc");

const files = computed(() => {
  let res = [...props.files];

  if(sortBy.value == "name") res.sort(compareName)
  else if(sortBy.value == "size") res.sort(compareSize);

  if (sortOrder.value == "desc") res.reverse();

  return res;
})

const compareName = (a: DirectoryItem, b: DirectoryItem) => {
  if (a.is_dir && !b.is_dir) return -1;
  if (!a.is_dir && b.is_dir) return 1;
  return a.name.localeCompare(b.name)
}

const compareSize = (a: DirectoryItem, b: DirectoryItem) => {
  if (a.is_dir && !b.is_dir) return -1;
  if (!a.is_dir && b.is_dir) return 1;
  return a.size - b.size
}

function sort(by: "name" | "size") {
  if (by != sortBy.value) {
    sortBy.value = by;
    return;
  }

  sortOrder.value = sortOrder.value == "asc" ? "desc" : "asc";
}

</script>

<template>
  <div class="w-full table px-1">
    <div class="table-header-group font-semibold">
      <div class="table-row">
        <div class="table-cell"/>
        <div class="table-cell" @click="sort('name')">
          <div class="flex justify-between">
            Name
            <template v-if="sortBy == 'name'">
              <IconCaretUpFilled v-if="sortOrder == 'desc'" :size="24"/>
              <IconCaretDownFilled v-if="sortOrder == 'asc'" :size="24"/>
            </template>
          </div>
        </div>
        <div class="table-cell" @click="sort('size')">
          <div class="flex justify-between">
            Size
            <template v-if="sortBy == 'size'">
              <IconCaretUpFilled v-if="sortOrder == 'desc'" :size="24"/>
              <IconCaretDownFilled v-else :size="24"/>
            </template>
          </div>
        </div>
      </div>
    </div>
    <template v-for="item in files">
      <DirectoryRow v-if="item.is_dir" :directory="item" @set-dir="path => $emit('setDir', path)"/>
      <FileRow v-else :file="item"/>
    </template>
  </div>
</template>