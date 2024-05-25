import { parseJwt } from "$lib/Utils";
import { writable } from "svelte/store";

export const auth = writable<{id?: string, profile_pic_url?: string, role?: string}>({id: undefined, profile_pic_url: undefined, role: undefined})

const gqlQuery = (queryString: string) => {
  const authToken = localStorage.getItem('sessionToken');
  return fetch(import.meta.env.VITE_SERVER_URL, {
    method: "post",
    headers: authToken ? {
      "Authorization": `Bearer ${authToken}`,
    } : {},
    body: JSON.stringify({ query: queryString }),
  });
};

export interface Visit {
  id: string;
  creatorId: string;
  arrival: string;
  departure: string;
  numStaying: number;
  postedOn: string;
}

export const getLedger = async (date: string) => {
  const query = `query {makeLedger(date: "${date}")}`;
  const request = await gqlQuery(query);
  const response = await request.json();
  return response.data.makeLedger;
};

export const addVisit = async (
  arrival: string,
  departure: string,
  numStaying: number,
) => {  
  const query = `mutation {addVisit(arrival: "${arrival}"
    departure:"${departure}" numStaying:${numStaying}){id}}`;
  await gqlQuery(query);
};

export const addBlankMember = async () => {
    const query = `mutation{addMember {id}}`;
    await gqlQuery(query);
}

export const removeMember = async (id: string) => {
    const query = `mutation{removeMember(id: "${id}") {id}}`;
    await gqlQuery(query);
}

export const updateMember = async (filter_id: string, credentials: string, role: string, name: string, email: string, phone: string, access_code: string) => {
    const query = `mutation{updateMember(filterId:"${filter_id}" accessCode:"${access_code}" role:${role} name:"${name}" email:"${email}" phone:"${phone}" credentials:"${credentials}"){id name email phone role sub accessCode}}`;
    await gqlQuery(query);
}

export const registerMember = async (access_code: string, credentials: string) => {
    const query = `mutation{registerMember(accessCode:"${access_code}" credentials:"${credentials}")}`;
    return (await gqlQuery(query)).json();
}
export type UserDict = {
  [name: string]: Omit<UserDTO, 'id'>;
}
export type UserDTO = {
			id: string;
			name: string;
			email: string;
			phone: string;
			role: string;
			sub: string;
			profilePic: string;
			accessCode: string;
		}
export const getMembers = async (): Promise<[UserDTO]> => {
    const query = `query{ getUsers {	id name email phone role sub profilePic accessCode }}`;
    const request = await gqlQuery(query);
    const response = await request.json();
    return response.data.getUsers;
}
export const getVisits = async (
  start?: string,
  end?: string,
): Promise<Visit[]> => {
  const query =
    `query{getVisits{ id creatorId arrival departure numStaying postedOn}}`;
  const request = await gqlQuery(query);
  const response = await request.json();
  return response.data.getVisits;
};

export const removeVisit = async (id: string) => {
  const query = `mutation{removeVisit(visitId: "${id}") {id}}`;
  await gqlQuery(query);
};

export interface Message {
  id: string;
  creatorId: string;
  name: string;
  profilePic: string;
  content: string;
  comments: Comment[];
  reactions: string[];
  seenBy: string[];
  postedOn: string;
}

export interface Comment {
  id: string;
  creatorId: string;
  content: string;
  reactions: string[];
}

export const addMessage = async (content: string) => {
  const query =
    `mutation {addMessage(content: "${content}") {id creatorId content seenBy postedOn}}`;
  await gqlQuery(query);
};

export const getMessages = async (): Promise<Message[]> => {
  const query =
    `query { getMessages {id creatorId comments{id creatorId content name profilePic reactions{id creatorId emoji}} reactions{id creatorId emoji} content seenBy postedOn name profilePic}}`;
  const request = await gqlQuery(query);
  const result = await request.json();
  return result.data.getMessages;
};

export const removeMessage = async (messageId: string) => {
  const query =
    `mutation {removeMessage(messageId: "${messageId}") {id creatorId content seenBy postedOn}}`;
  await gqlQuery(query);
};

export const addComment = async (messageId: string, commentContent: string) => {
  const query =
    `mutation {addComment(messageId: "${messageId}" commentContent:"${commentContent}"){id}}`;
  await gqlQuery(query);
};

export const removeComment = async (messageId: string, commentId: string) => {
  const query =
    `mutation {removeComment(messageId: "${messageId}" commentId:"${commentId}"){id}}`;
  await gqlQuery(query);
};

export const getSession = async (credential: string) => {
  const query = `mutation {getSession(credentials: "${credential}")}`;
  const request = await gqlQuery(query);
  const response = await request.json();
  return response.data.getSession;
};
