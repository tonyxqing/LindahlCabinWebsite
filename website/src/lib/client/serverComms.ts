 const gqlQuery = (queryString: string) => fetch("http://localhost:8000", {
    method: 'post',
    headers: {},
    body: JSON.stringify({query: queryString})
 })

 export interface Visit {
   id: string,
   creatorId: string,
   arrival: string,
   departure: string,
   numStaying: number,
   postedOn: string
 }

 export const addVisit = async (arrival: string, departure: string, numStaying: number) => {
    const query = `mutation {addVisit(creatorId: "0000" arrival: "${arrival}"
    departure:"${departure}" numStaying:${numStaying}){id}}`;
    await gqlQuery(query);
 }

 export const getVisits = async (start?: string, end?: string): Promise<Visit[]> => {
   const query = `query{getVisits{ id creatorId arrival departure numStaying postedOn}}`
   const request = await gqlQuery(query);
   const response = await request.json()
   return response.data.getVisits
 }
 
 export const removeVisit = async (id: string) => {
   const query = `mutation{removeVisit(visitId: "${id}") {id}}`
   await gqlQuery(query);
 }

 export interface Message {
   id: string,
   creatorId: string,
   content: string,
   comments: Comment[],
   reactions: string[],
   seenBy: string[],
   postedOn: string,
 }

 export interface Comment {
   id: string,
   creatorId: string,
   content: string,
   reactions: string[],
 }

 export const addMessage = async (creatorId: string, content: string) => {
   const query = `mutation {addMessage(creatorId: "${creatorId}" content: "${content}") {id creatorId content seenBy postedOn}}`
   await gqlQuery(query);
 }

 export const getMessages = async (): Promise<Message[]> => {
   const query = `query { getMessages {id creatorId comments{id creatorId content reactions{id creatorId emoji}} reactions{id creatorId emoji} content seenBy postedOn}}`
   const request = await gqlQuery(query);
   const result = await request.json();
   return result.data.getMessages;
}

 export const removeMessage = async (messageId: string) => {
   const query = `mutation {removeMessage(messageId: "${messageId}") {id creatorId content seenBy postedOn}}`
   await gqlQuery(query);
 }

 export const addComment = async (messageId: string, commentContent: string) => {
   const query = `mutation {addComment(messageId: "${messageId}" commentContent:"${commentContent}"){id}}`
   await gqlQuery(query);
}

 export const removeComment = async (messageId: string, commentId: string) => {
   const query = `mutation {removeComment(messageId: "${messageId}" commentId:"${commentId}"){id}}`
   await gqlQuery(query);
 }

 export const getSession = async (credential: string) => {
  const query = `mutation {getSession(credentials: "${credential}")}`
  const request = await gqlQuery(query);
  const response = await request.json();
  return response.data.getSession;

 }