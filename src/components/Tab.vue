<script setup lang="ts">
import {Ref, ref, watch} from "vue";
import {DirectoryItem} from "../models/DirectoryItem.ts";
import {invoke} from "@tauri-apps/api/tauri";
import {IconSearch, IconChevronLeft, IconChevronRight, IconLayoutGridFilled, IconList} from '@tabler/icons-vue';
import GridView from "./GridView.vue";
import TableView from "./TableView.vue";

const emits = defineEmits<{locationUpdated: [value: string]}>();

const current_path: Ref<string> = ref("");
const display: Ref<"grid"|"list"> = ref("grid");
const files: Ref<Array<DirectoryItem> | null> = ref(null);

const before: string[] = [];
let after: string[] = [];

invoke("home_dir_path").then(async value => {
  current_path.value = <string>value
});

function nextDisplay(){
  if(display.value == "grid") display.value = "list";
  else display.value = "grid";
}

watch(current_path, async () => {
  await loadDir();
  emits("locationUpdated", current_path.value);
});

async function loadDir() {
  files.value = await invoke("read_dir", {
    path: current_path.value,
    with_hidden: true
  });
}

async function setDir(path: string) {
  before.push(current_path.value);
  after = [];

  current_path.value = path;
}

async function back() {
  if (before.length <= 0) {
    return;
  }

  after.push(current_path.value);
  current_path.value = before.pop() ?? "";
}

async function forward() {
  if (after.length <= 0) {
    return;
  }

  before.push(current_path.value);
  current_path.value = after.pop() ?? "";
}

</script>

<template>
  <div>
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
      <button @click="nextDisplay">
        <IconLayoutGridFilled v-if="display == 'grid'" size="32"/>
        <IconList v-else-if="display == 'list'" size="32" stroke="2"/>
      </button>
    </div>
    <p v-if="current_path == null || files == null" class="m-1">Directory not found.</p>
    <p v-else-if="files.length <= 0" class="m-1">Empty directory.</p>
    <GridView v-else-if="display == 'grid'" :files="files" @set-dir="path => setDir(path)"/>
    <TableView v-else :files="files" @set-dir="path => setDir(path)"/>
  </div>
</template>