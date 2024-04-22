<script setup lang="ts">
import {Ref, ref} from "vue";
import {DirectoryItem} from "../models/DirectoryItem.ts";
import {invoke} from "@tauri-apps/api/tauri";
import {IconSearch, IconChevronLeft, IconChevronRight} from '@tabler/icons-vue';
import FileCard from "./FileCard.vue";
import DirectoryCard from "./DirectoryCard.vue";


const current_path: Ref<string> = ref("");

invoke("home_dir_path").then(async value => {
  current_path.value = <string>value
  await loadDir()
});

const files: Ref<Array<DirectoryItem> | null> = ref(null);
const before: string[] = [];
let after: string[] = [];


async function loadDir() {
  let unordered_files: Array<DirectoryItem> | null = await invoke("read_dir", {
    path: current_path.value,
    with_hidden: true
  });

  if (unordered_files == null) {
    files.value = null;
    return;
  }

  files.value = unordered_files.sort((a, b) => a.name.localeCompare(b.name))
}

async function setDir(path: string) {
  before.push(current_path.value);
  current_path.value = path;
  await loadDir();

  after = [];
}

async function back() {
  if (before.length <= 0) {
    return;
  }

  after.push(current_path.value);
  current_path.value = before.pop() ?? "";
  await loadDir();
}

async function forward() {
  if (after.length <= 0) {
    return;
  }

  before.push(current_path.value);
  current_path.value = after.pop() ?? "";
  await loadDir();
}

</script>

<template>
  <div class="flex w-full gap-1 p-1">
    <div class="flex">
      <button class="border rounded-l" @click="back">
        <IconChevronLeft :size="24"/>
      </button>
      <button class="border rounded-r" @click="forward">
        <IconChevronRight :size="24"/>
      </button>
    </div>
    <form class="grow" @submit.prevent="loadDir">
      <div class="relative w-full">
        <input v-model="current_path"
               class="block p-2.5 w-full z-20 text-sm text-gray-900 bg-gray-50 rounded-s-gray-100 rounded-s-2 border border-gray-300 focus:ring-blue-500 focus:border-blue-500 rounded"/>
        <button type="submit"
                class="absolute top-0 end-0 p-2.5 h-full text-sm font-medium text-white bg-blue-700 border border-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 rounded-r">
          <IconSearch color="white" size="20" stroke-width="2"/>
        </button>
      </div>
    </form>
  </div>

  <p v-if="current_path == null || files == null" class="m-1">Directory not found.</p>
  <p v-else-if="files.length <= 0" class="m-1">Empty directory.</p>
  <div v-else class="w-full grid gap-2 grid-template-100">
    <div v-for="item in files">
      <DirectoryCard v-if="item.is_dir" :directory="item" @go-to-dir="setDir"/>
      <FileCard v-else :file="item"/>
    </div>
  </div>
</template>

<style scoped>
.grid-template-100 {
  grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
}
</style>