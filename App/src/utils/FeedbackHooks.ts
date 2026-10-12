import { useFeedbackStore, FeedbackType } from "@/store/FeedbackStore";
import { useBauwerkStore } from "@/store/BauwerkStore";


const { setBauwerkList } = useBauwerkStore();


export const getBauwerkeListWithFeedback = async () => {
  try {
    await setBauwerkList();
  } catch (error: any) {
      useFeedbackStore().addFeedback({
        message: error.message ?? "Bauwerke konnten nicht geladen werden",
        type: FeedbackType.ERROR,
        action: "Bauwerke laden"
    });
  }
}
