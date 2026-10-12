import { ref, type Ref } from "vue";
import { defineStore } from "pinia";


export enum FeedbackType {
    SUCCESS = 'success',
    ERROR = 'error',
    WARNING = 'warning',
    INFO = 'info'
}
    
export interface IFeedback {
    action: string;
    message: string;
    type: FeedbackType;
    id: string;  // is just use as a handle for auto-removing from the feed
}

type IFeedbackPreview = Omit<IFeedback, 'id'>;


export const useFeedbackStore = defineStore('feedback', () => {
    const feedbackFeed: Ref<IFeedback[]> = ref([]);
  
    function addError(message: string, action: string = '') {
        addFeedback({ message, action, type: FeedbackType.ERROR });
    }
    function addSuccess(message: string, action: string = '') {
        addFeedback({ message, action, type: FeedbackType.SUCCESS });
    }
    function addWarning(message: string, action: string = '') {
        addFeedback({ message, action, type: FeedbackType.WARNING });
    }
    function addInfo(message: string, action: string = '') {
        addFeedback({ message, action, type: FeedbackType.INFO });
    }

    function addFeedback(feedbackPreview: IFeedbackPreview) {
        const feedback: IFeedback = { ...feedbackPreview, id: generateFeedbackId() };
        // add feedback to the feed which will display it to the user
        feedbackFeed.value.push(feedback);
        // remove feedback from the feed after 10 seconds
        setTimeout(() => {
          removeFeedback(feedback.id);
        }, 10000);
    }

    function removeFeedback(id: string) {
        const index = feedbackFeed.value.findIndex((feedback) => feedback.id === id);
        if (index === -1) {
            // feedback already removed (e.g. by clicking the dismiss button)
            return;
        }
        feedbackFeed.value.splice(index, 1);
    }
    
    function generateFeedbackId(): string {
        const feedbackId = Math.random().toString().slice(2);  // good enough
        // generate new id if it is already assigned
        if (feedbackId in feedbackFeed.value.map(feedback => feedback.id)) {
          return generateFeedbackId();
        }
        return feedbackId
    }

    return { feedbackFeed, addFeedback, removeFeedback, addError, addSuccess, addWarning, addInfo };
});
