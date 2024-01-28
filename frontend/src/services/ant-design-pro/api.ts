// @ts-ignore
/* eslint-disable */
import { request } from '@umijs/max';

/** 获取当前的用户 GET /v1/users/me */
export async function currentUser(options?: { [key: string]: any }) {
  return request<{
    data: API.User;
  }>('/v1/users/me', {
    method: 'GET',
    ...(options || {}),
  });
}

/** 退出登录接口 POST /logout */
export async function logout(options?: { [key: string]: any }) {
  const res = await request<Record<string, any>>('/logout', {
    method: 'GET',
    ...(options || {}),
  });

  if (res.code === 0) {
    localStorage.removeItem('token');
  }

  return res;
}

/** 登录接口 POST /login */
export async function login(body: API.LoginParams, options?: { [key: string]: any }) {
  const res = await request<API.LoginResult>('/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    data: body,
    ...(options || {}),
  });

  if (res.code === 0 && res.data) {
    localStorage.setItem('token', res.data);
  }

  return res;
}

/** 获取用户列表 GET /v1/users */
export async function listUsers(
  params: {
    // query
    /** 当前的页码 */
    current?: number;
    /** 页面的容量 */
    pageSize?: number;
    withGroups?: boolean;
    withPermissions?: boolean;
  },
  options?: { [key: string]: any },
) {
  const res = await request<API.User>('/v1/users', {
    method: 'GET',
    params: {
      skip: params.current && params.pageSize ? (params.current - 1) * params.pageSize : 0,
      limit: params.pageSize ?? 20,
      groups: params.withGroups,
      permissions: params.withPermissions,
    },
    ...(options || {}),
  });

  return {
    success: true,
    ...res,
  }
}

/** 新建用户 POST /v1/users */
export async function addUser(options?: { [key: string]: any }) {
  return request<API.User>('/v1/users', {
    method: 'POST',
    data: {
      method: 'post',
      ...(options || {}),
    }
  });
}

/** 更新用户 PUT /v1/users */
export async function updateUser(
  params: {
    userId: number;
    updatedUser: API.User & { password?: string };
  },
  options?: { [key: string]: any },
) {
  return request<API.User>(`/v1/users/${params.userId}`, {
    method: 'PUT',
    data: {
      method: 'put',
      ...params.updatedUser,
      ...(options || {}),
    }
  });
}

/** 激活用户 GET /v1/users/{id}/activate */
export async function activateUser(
  params: {
    userId: number;
  },
  options?: { [key: string]: any },
) {
  return request<API.User>(`/v1/users/${params.userId}/activate`, {
    method: 'GET',
    ...(options || {}),
  });
}

/** 停用用户 GET /v1/users/{id}/deactivate */
export async function deactivateUser(
  params: {
    userId: number;
  },
  options?: { [key: string]: any }
) {
  return request<API.User>(`/v1/users/${params.userId}/deactivate`, {
    method: 'GET',
    ...(options || {}),
  });
}

// deleteUser
/** 删除用户 DELETE /v1/users/ */
export async function deleteUsers(
  params: {
    ids: number[];
  },
  options?: { [key: string]: any }
) {
  return request<API.User>(`/v1/users`, {
    method: 'DELETE',
    data: {
      ids: params.ids,
    },
    ...(options || {}),
  });
}

/** 新建组 POST /v1/groups */
export async function addGroup(options?: { [key: string]: any }) {
  return request<API.User>('/v1/groups', {
    method: 'POST',
    data: {
      method: 'post',
      ...(options || {}),
    }
  });
}

/** 获取组列表 GET /v1/groups */
export async function listGroups(
  params: {
    // query
    /** 当前的页码 */
    current?: number;
    /** 页面的容量 */
    pageSize?: number;
  },
  options?: { [key: string]: any },
) {
  return request<API.User>('/v1/groups', {
    method: 'GET',
    params: {
      skip: params.current && params.pageSize ? (params.current - 1) * params.pageSize : 0,
      limit: params.pageSize ?? 20,
    },
    ...(options || {}),
  });
}

// deleteUser
/** 删除组 DELETE /v1/groups */
export async function deleteGroups(
  params: {
    ids: number[];
  },
  options?: { [key: string]: any }
) {
  return request<API.User>(`/v1/groups`, {
    method: 'DELETE',
    data: {
      ids: params.ids,
    },
    ...(options || {}),
  });
}

/** 新建组 POST /v1/permissions */
export async function addPermission(options?: { [key: string]: any }) {
  return request<API.User>('/v1/permissions', {
    method: 'POST',
    data: {
      method: 'post',
      ...(options || {}),
    }
  });
}

/** 获取组列表 GET /v1/permissions */
export async function listPermissions(
  params: {
    // query
    /** 当前的页码 */
    current?: number;
    /** 页面的容量 */
    pageSize?: number;
  },
  options?: { [key: string]: any },
) {
  return request<API.User>('/v1/permissions', {
    method: 'GET',
    params: {
      skip: params.current && params.pageSize ? (params.current - 1) * params.pageSize : 0,
      limit: params.pageSize ?? 20,
    },
    ...(options || {}),
  });
}

// deletePermissions
/** 删除权限 GET /v1/permissions */
export async function deletePermissions(
  params: {
    ids: number[];
  },
  options?: { [key: string]: any }
) {
  return request<API.User>(`/v1/permissions`, {
    method: 'DELETE',
    data: {
      ids: params.ids,
    },
    ...(options || {}),
  });
}

/** 此处后端没有提供注释 GET /api/notices */
export async function getNotices(options?: { [key: string]: any }) {
  return request<API.NoticeIconList>('/api/notices', {
    method: 'GET',
    ...(options || {}),
  });
}

/** 获取规则列表 GET /api/rule */
export async function rule(
  params: {
    // query
    /** 当前的页码 */
    current?: number;
    /** 页面的容量 */
    pageSize?: number;
  },
  options?: { [key: string]: any },
) {
  return request<API.RuleList>('/api/rule', {
    method: 'GET',
    params: {
      ...params,
    },
    ...(options || {}),
  });
}

/** 更新规则 PUT /api/rule */
export async function updateRule(options?: { [key: string]: any }) {
  return request<API.RuleListItem>('/api/rule', {
    method: 'POST',
    data:{
      method: 'update',
      ...(options || {}),
    }
  });
}

/** 新建规则 POST /api/rule */
export async function addRule(options?: { [key: string]: any }) {
  return request<API.RuleListItem>('/api/rule', {
    method: 'POST',
    data:{
      method: 'post',
      ...(options || {}),
    }
  });
}

/** 删除规则 DELETE /api/rule */
export async function removeRule(options?: { [key: string]: any }) {
  return request<Record<string, any>>('/api/rule', {
    method: 'POST',
    data:{
      method: 'delete',
      ...(options || {}),
    }
  });
}
