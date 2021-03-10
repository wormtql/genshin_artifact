<template>
    <div class="choose-artifact-position">
        <div
            v-for="position in availablePositions"
            :key="position"
            :class="{active: position === value}"
            class="item"
            @click="$emit('input', position)"
        >
            <img
                :src="selectedArtData[position].url"
                class="image"
            >
            <p class="text">
                {{ selectedArtData[position].chs }}
            </p>
        </div>
    </div>
</template>

<script>
import { artifactsData } from "../../../assets/artifacts";

export default {
    name: "PositionChoose",
    created: function () {
        this.artifactsData = artifactsData;
    },
    props: {
        setName: {
            type: String,
        },
        value: {
            type: String
        }
    },
    methods: {
        choose(position) {
            this.$emit("input", position);
        }
    },
    computed: {
        selectedArtData() {
            return this.artifactsData[this.setName] || {};
        },

        availablePositions() {
            return ["flower", "feather", "sand", "cup", "head"].filter((value) => {
                return Object.prototype.hasOwnProperty.call(this.selectedArtData, value);
            });
        }
    },
    watch: {
        setName() {
            if (this.availablePositions.indexOf(this.value) === -1) {
                // the last selected position does not exist on current set
                let autoSelectedPosition = this.availablePositions[0];
                this.$emit("input", autoSelectedPosition);
            }
        }
    }
}
</script>

<style scoped>
.choose-artifact-position {
    display: flex;
    flex-wrap: wrap;
}

.item {
    padding: 8px;
    color: #777777;
    margin-right: 4px;
}

.item:hover {
    background: rgba(0, 0, 0, 0.1);
    cursor: pointer;
    border-radius: 3px;
    transition: 300ms;
}

.item.active {
    /* background: rgba(0, 0, 0, 0.1); */
    border-radius: 3px;
    /* color: white; */
    background: none;
    box-shadow: 0 0 20px 1px #00000033;
}

.text {
    padding: 0;
    margin: 0;
    font-size: 12px;
    text-align: center;
    width: 48px;
}

.image {
    height: 48px;
    width: 48px;
    border-radius: 50%;
}
</style>