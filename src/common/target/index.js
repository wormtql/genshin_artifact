import { plans as keqingPlan } from "./keqing";
import { plans as dilukePlan } from "./diluke";
import { plans as commonPlans } from "./single";
import { plans as qiqiPlans } from "./qiqi";

const cPlans = [];
cPlans.push(...keqingPlan);
cPlans.push(...dilukePlan);
cPlans.push(...qiqiPlans);

export const plans = {
    "character": cPlans,
    "common": commonPlans,
};

export function getTargetFunction(value) {
    for (let key in plans) {
        for (let i = 0; i < plans[key].length; i++) {
            if (plans[key][i].value === value) {
                return plans[key][i].calc;
            }
        }
    }
    
    return null;
}