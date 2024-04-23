<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Tab from "./components/Tab.vue";
import {Ref, ref} from "vue";
import {IconX, IconPlus} from "@tabler/icons-vue";

const tabs: Ref<Map<number, { location: string, name: string }>> = ref(new Map([[Date.now(), {
  location: "",
  name: ""
}]]))

const current_tab = ref(tabs.value.keys().next().value)

function addTab() {
  tabs.value.set(Date.now(), {location: "", name: ""})
}

function removeTab(tab: number) {
  tabs.value.delete(tab);
  if (tabs.value.size <= 0) {
    addTab()
  }
  if (!tabs.value.has(current_tab.value)) {
    current_tab.value = tabs.value.keys().next().value
  }
}

function updateTab(tab: number, location: string) {
  tabs.value.set(tab, {location, name: location.split("/").pop() ?? ""})
}

</script>

<template>
  <div class="h-screen flex flex-col bg-gray-200">
    <div class="shrink-0 flex gap-1 px-1 pt-1 overflow-x-scroll">
      <div v-for="[tab, data] in tabs" :key="tab"
           @click="current_tab = tab"
           class="rounded-t p-1 text-nowrap cursor-pointer flex items-center gap-2"
           :class="{'bg-white pb-2': tab === current_tab, 'bg-gray-100 rounded-b mb-1 hover:bg-white': tab !== current_tab}">
        {{ data.name }}
        <button @click.stop="removeTab(tab)" class="rounded-full p-0.5 hover:bg-black/10">
          <IconX size="16" stroke="2"/>
        </button>
      </div>
      <button @click="addTab" class="rounded p-1 hover:bg-black/10"><IconPlus size="16" stroke="2"/></button>
    </div>
    <div class="grow overflow-y-scroll bg-white">
      <Tab v-for="tab in tabs.keys()" :key="tab"
           @location-updated="value => updateTab(tab, value)"
           :class="{'hidden': tab !== current_tab}"/>
    </div>
  </div>
</template>

<style scoped>
</style>
