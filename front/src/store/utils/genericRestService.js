import axios from 'axios'

function generateErrorLog (commit, error) {
  console.error('Oups .... ', error)
  commit('raiseError', error, { root: true }) // Be careful a global raiseError mutation and a lastError state should be defined in your store
}

async function fetchItem (commit, params, baseUrl, postAction, postActionClone = '', rethrow = false) {
  const url = params.id ? baseUrl + '/' + params.id : baseUrl

  try {
    commit('startApiCall')
    const resp = await axios.get(url, {
      params: params
    })
    commit('endApiCall')
    if (postAction) {
      commit(postAction, resp.data)
    }
    if (postActionClone) {
      commit(postActionClone, resp.data)
    }
    return resp
  } catch (error) {
    generateErrorLog(commit, error)
    if (rethrow) {
      throw error
    }
  } finally {
    commit('endApiCall')
  }
}

async function fetchItems (commit, params, baseUrl, postAction) {
  try {
    commit('startApiCall')
    const resp = await axios.get(baseUrl, {
      params: params
    })
    commit('endApiCall')
    if (postAction) {
      commit(postAction, resp.data)
    }
    return resp
  } catch (error) {
    generateErrorLog(commit, error)
  } finally {
    commit('endApiCall')
  }
}

async function fetchPaginatedItems (commit, params, baseUrl, postAction) {
  try {
    commit('startApiCall')
    const resp = await axios.get(baseUrl, {
      params: params
    })
    commit(postAction, resp.data)
    return resp.data
  } catch (error) {
    generateErrorLog(commit, error)
  } finally {
    commit('endApiCall')
  }
}

async function storeItem (commit, params, baseUrl, postAction) {
  try {
    commit('startApiCall')
    const resp = await axios.post(baseUrl, params)
    commit('endApiCall')
    if (postAction) {
      commit(postAction, resp.data)
    }
    return resp
  } catch (error) {
    commit('endApiCall')
    generateErrorLog(commit, error)
    throw error
  }
}

async function storeFile (commit, formData, baseUrl, postAction) {
  try {
    commit('startApiCall')
    const resp = axios.post(baseUrl, formData, {
        headers: {
          'Content-Type': 'multipart/form-data'
        }
    })
    commit('endApiCall')
    if (postAction) {
      commit(postAction, resp.data)
    }
    return resp
  } catch (error) {
    commit('endApiCall')
    generateErrorLog(commit, error)
    throw error
  }
}

async function updateItem (commit, params, baseUrl, postAction, postActionClone = '') {
  const url = params.id ? baseUrl + '/' + params.id : baseUrl

  try {
    commit('startApiCall')
    const resp = await axios.put(url, params)
    commit('endApiCall')
    if (postAction) {
      commit(postAction, resp.data)
    }
    if (postActionClone) {
      commit(postActionClone, resp.data)
    }
    return resp
  } catch (error) {
    commit('endApiCall')
    generateErrorLog(commit, error)
    throw error
  }
}

async function deleteItem (commit, params, baseUrl, postAction) {
  try {
    commit('startApiCall')
    const resp = await axios.delete(baseUrl + (params.id ? '/' + params.id : ''), params)
    commit('endApiCall')
    if (postAction) {
      commit(postAction, resp.data)
    }
    return resp
  } catch (error) {
    commit('endApiCall')
    generateErrorLog(commit, error)
    throw error
  }
}

const defaultMutation = {
  startApiCall (state) {
    state.loading = true
  },
  endApiCall (state) {
    state.loading = false
  }
}

const defaultState = {
  loading: false
}

export { fetchItem, fetchItems, fetchPaginatedItems, storeItem, storeFile, updateItem, deleteItem, generateErrorLog, defaultMutation, defaultState }
