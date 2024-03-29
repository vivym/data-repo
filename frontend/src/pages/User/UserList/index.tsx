import { addUser, activateUser, deactivateUser, deleteUsers, listUsers, updateUser } from '@/services/ant-design-pro/api';
import { PlusOutlined } from '@ant-design/icons';
import type { ActionType, ProColumns, ProDescriptionsItemProps } from '@ant-design/pro-components';
import {
  FooterToolbar,
  ModalForm,
  PageContainer,
  ProDescriptions,
  ProFormText,
  ProTable,
} from '@ant-design/pro-components';
import { FormattedMessage, useIntl } from '@umijs/max';
import { Button, Drawer, message } from 'antd';
import React, { useRef, useState } from 'react';
import type { FormValueType } from './components/UpdateForm';
import UpdateForm from './components/UpdateForm';

/**
 * @en-US Add node
 * @zh-CN 添加节点
 * @param fields
 */
const handleAdd = async (fields: API.User & { password: string }) => {
  const hide = message.loading('正在添加');
  try {
    await addUser({
      username: fields.username,
      nickname: fields.nickname,
      avatar_uri: fields.avatar_uri,
      password: fields.password,
    });
    hide();
    message.success('Added successfully');
    return true;
  } catch (error) {
    hide();
    message.error('Adding failed, please try again!');
    return false;
  }
};

/**
 * @en-US Update node
 * @zh-CN 更新节点
 *
 * @param fields
 */
const handleUpdate = async (fields: FormValueType) => {
  const hide = message.loading('Configuring');
  try {
    await updateUser({
      userId: fields.id!!,
      updatedUser: {
        password: fields.password,
        nickname: fields.nickname,
        avatar_uri: fields.avatar_uri,
      },
    });
    hide();

    message.success('Configuration is successful');
    return true;
  } catch (error) {
    hide();
    message.error('Configuration failed, please try again!');
    return false;
  }
};

/**
 *  Delete node
 * @zh-CN 删除节点
 *
 * @param selectedRows
 */
const handleRemove = async (selectedRows: API.User[]) => {
  const hide = message.loading('正在删除');
  if (!selectedRows) return true;
  try {
    await deleteUsers({
      ids: selectedRows.map((row) => row.id!!),
    });
    hide();
    message.success('Deleted successfully and will refresh soon');
    return true;
  } catch (error) {
    hide();
    message.error('Delete failed, please try again');
    return false;
  }
};

const UserList: React.FC = () => {
  /**
   * @en-US Pop-up window of new window
   * @zh-CN 新建窗口的弹窗
   *  */
  const [createModalOpen, handleModalOpen] = useState<boolean>(false);
  /**
   * @en-US The pop-up window of the distribution update window
   * @zh-CN 分布更新窗口的弹窗
   * */
  const [updateModalOpen, handleUpdateModalOpen] = useState<boolean>(false);

  const [showDetail, setShowDetail] = useState<boolean>(false);

  const actionRef = useRef<ActionType>();
  const [currentRow, setCurrentRow] = useState<API.User>();
  const [selectedRowsState, setSelectedRows] = useState<API.User[]>([]);

  /**
   * @en-US International configuration
   * @zh-CN 国际化配置
   * */
  const intl = useIntl();

  const columns: ProColumns<API.User>[] = [
    {
      title: (
        <FormattedMessage
          id="pages.userList.colums.userId"
          defaultMessage="User ID"
        />
      ),
      dataIndex: 'id',
      tip: 'The User ID is the unique key',
      render: (dom, entity) => {
        return (
          <a
            onClick={() => {
              setCurrentRow(entity);
              setShowDetail(true);
            }}
          >
            {dom}
          </a>
        );
      },
    },
    {
      title: <FormattedMessage id="pages.userList.colums.username" defaultMessage="Username" />,
      dataIndex: 'username',
      // valueType: 'textarea',
    },
    {
      title: <FormattedMessage id="pages.userList.colums.status" defaultMessage="Status" />,
      dataIndex: 'is_active',
      hideInForm: true,
      valueEnum: {
        false: {
          text: (
            <FormattedMessage
              id="pages.userList.colums.status.inactive"
              defaultMessage="Inactive"
            />
          ),
          status: 'Default',
        },
        true: {
          text: (
            <FormattedMessage id="pages.userList.colums.status.active" defaultMessage="Online" />
          ),
          status: 'Success',
        },
      },
    },
    {
      title: (
        <FormattedMessage
          id="pages.userList.colums.status.createdAt"
          defaultMessage="Created at"
        />
      ),
      sorter: true,
      dataIndex: 'created_at',
      valueType: 'dateTime',
    },
    {
      title: (
        <FormattedMessage
          id="pages.userList.colums.status.updatedAt"
          defaultMessage="Updated at"
        />
      ),
      sorter: true,
      dataIndex: 'updated_at',
      valueType: 'dateTime',
    },
    {
      title: <FormattedMessage id="pages.searchTable.titleOption" defaultMessage="Operating" />,
      dataIndex: 'option',
      valueType: 'option',
      render: (_, record) => [
        <a
          key="edit-user"
          onClick={() => {
            handleUpdateModalOpen(true);
            setCurrentRow(record);
          }}
        >
          <FormattedMessage id="pages.userList.colums.op.edit" defaultMessage="Edit" />
        </a>,
        <a
          key="activate-user"
          onClick={(e) => {
            e.preventDefault();
            if (record.is_active) {
              deactivateUser({ userId: record.id!! });
            } else {
              activateUser({ userId: record.id!! });
            }
            if (actionRef.current) {
              actionRef.current.reload();
            }
          }}
        >
          {record.is_active ? (
            <FormattedMessage
              id="pages.userList.colums.op.deactivate"
              defaultMessage="Deactivate"
            />
          ) : (
            <FormattedMessage
              id="pages.userList.colums.op.activate"
              defaultMessage="Activate"
            />
          )}
        </a>,
      ],
    },
  ];

  return (
    <PageContainer>
      <ProTable<API.User, API.PageParams>
        headerTitle={intl.formatMessage({
          id: 'pages.searchTable.title',
          defaultMessage: 'Enquiry form',
        })}
        actionRef={actionRef}
        rowKey="id"
        search={{
          labelWidth: 120,
        }}
        toolBarRender={() => [
          <Button
            type="primary"
            key="primary"
            onClick={() => {
              handleModalOpen(true);
            }}
          >
            <PlusOutlined /> <FormattedMessage id="pages.searchTable.new" defaultMessage="New" />
          </Button>,
        ]}
        request={listUsers}
        columns={columns}
        rowSelection={{
          onChange: (_, selectedRows) => {
            setSelectedRows(selectedRows);
          },
        }}
      />
      {selectedRowsState?.length > 0 && (
        <FooterToolbar
          extra={
            <div>
              <FormattedMessage id="pages.searchTable.chosen" defaultMessage="Chosen" />{' '}
              <a style={{ fontWeight: 600 }}>{selectedRowsState.length}</a>{' '}
              <FormattedMessage id="pages.searchTable.item" defaultMessage="项" />
            </div>
          }
        >
          <Button
            onClick={async () => {
              await handleRemove(selectedRowsState);
              setSelectedRows([]);
              actionRef.current?.reloadAndRest?.();
            }}
          >
            <FormattedMessage
              id="pages.searchTable.batchDeletion"
              defaultMessage="Batch deletion"
            />
          </Button>
          <Button type="primary">
            <FormattedMessage
              id="pages.userList.batchActivate"
              defaultMessage="Batch Activate"
            />
          </Button>
          <Button type="primary">
            <FormattedMessage
              id="pages.userList.batchDeactivate"
              defaultMessage="Batch Deactivate"
            />
          </Button>
        </FooterToolbar>
      )}
      <ModalForm
        title={intl.formatMessage({
          id: 'pages.userList.createForm.newUser',
          defaultMessage: 'New User',
        })}
        width="400px"
        open={createModalOpen}
        onOpenChange={handleModalOpen}
        onFinish={async (value) => {
          const success = await handleAdd(value as API.User & { password: string });
          if (success) {
            handleModalOpen(false);
            if (actionRef.current) {
              actionRef.current.reload();
            }
          }
        }}
      >
        <ProFormText
          label={intl.formatMessage({
            id: 'pages.userList.createForm.username',
            defaultMessage: 'Username',
          })}
          rules={[
            {
              required: true,
              min: 3,
              max: 20,
              message: (
                <FormattedMessage
                  id="pages.userList.createForm.username.tooltip"
                  defaultMessage="Username is required (3 ~ 20 chars)"
                />
              ),
            },
          ]}
          width="md"
          name="username"
        />
        <ProFormText
          label={intl.formatMessage({
            id: 'pages.userList.createForm.nickname',
            defaultMessage: 'Nickname',
          })}
          rules={[
            {
              required: true,
              min: 3,
              max: 20,
              message: (
                <FormattedMessage
                  id="pages.userList.createForm.nickname.tooltip"
                  defaultMessage="Nickname is required (3 ~ 20 chars)"
                />
              ),
            },
          ]}
          width="md"
          name="nickname"
        />
        <ProFormText.Password
          label={intl.formatMessage({
            id: 'pages.userList.createForm.password',
            defaultMessage: 'Password',
          })}
          rules={[
            {
              required: true,
              min: 8,
              max: 20,
              message: (
                <FormattedMessage
                  id="pages.userList.createForm.password.tooltip"
                  defaultMessage="Password is required (8 ~ 20 chars)"
                />
              ),
            },
          ]}
          width="md"
          name="password"
        />
        <ProFormText
          label={intl.formatMessage({
            id: 'pages.userList.createForm.avatarUri',
            defaultMessage: 'Avatar URI',
          })}
          rules={[
            {
              required: true,
              min: 3,
              max: 20,
              type: 'url',
              message: (
                <FormattedMessage
                  id="pages.userList.createForm.avatarUri.tooltip"
                  defaultMessage="Avatar URI is required to be a valid URL"
                />
              ),
            },
          ]}
          width="xl"
          name="avatar_uri"
        />
      </ModalForm>
      <UpdateForm
        onSubmit={async (value) => {
          const success = await handleUpdate(value);
          if (success) {
            handleUpdateModalOpen(false);
            setCurrentRow(undefined);
            if (actionRef.current) {
              actionRef.current.reload();
            }
          }
        }}
        onCancel={() => {
          handleUpdateModalOpen(false);
          if (!showDetail) {
            setCurrentRow(undefined);
          }
        }}
        updateModalOpen={updateModalOpen}
        values={currentRow || {}}
      />

      <Drawer
        width={600}
        open={showDetail}
        onClose={() => {
          setCurrentRow(undefined);
          setShowDetail(false);
        }}
        closable={false}
      >
        {currentRow?.id && (
          <ProDescriptions<API.User>
            column={2}
            title={currentRow?.username}
            request={async () => ({
              data: currentRow || {},
            })}
            params={{
              id: currentRow?.id,
            }}
            columns={columns as ProDescriptionsItemProps<API.User>[]}
          />
        )}
      </Drawer>
    </PageContainer>
  );
};

export default UserList;
