<script lang="ts">
  import Tabs from "../components/Tabs.svelte";

    import ConnectRandomServerSection from "./ConnectRandomServerSection.svelte";
  import ConnectCountrySection from "./ConnectCountrySection.svelte";
  import ConnectCitySection from "./ConnectCitySection.svelte";
  import ConnectGroupSection from "./ConnectGroupSection.svelte";


  let connectSectionItems = [
    { label: "Random", value: 1, component: ConnectRandomServerSection },
    { label: "Country", value: 2, component: ConnectCountrySection },
    { label: "City", value: 3, component: ConnectCitySection },
    { label: "Group", value: 4, component: ConnectGroupSection },
  ];
</script>


   <Tabs items={connectSectionItems} on:SERVER_CONNECTION_ATTEMPTED />
