import { getCoordinatorApi } from "~/services/api";
import type {ActionFunctionArgs} from "react-router";

export async function action({ request }: ActionFunctionArgs) {
  try {
    const coordinatorApi = await getCoordinatorApi(request);
    const body = await request.json();
    
    const response = await coordinatorApi.deviceSign.start({
      device_id: body.device_id,
    });
    
    return Response.json(response);
  } catch (error: any) {
    return Response.json(
      { message: error.message || 'Failed to start device signing' },
      { status: error.status || 500 }
    );
  }
}