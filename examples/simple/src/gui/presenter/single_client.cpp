// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "single_client.h"
#include "client/client_controller.h"
#include "event_dispatcher.h"

using namespace Simple::Controller;
using namespace Simple::Presenter;

SingleClient::SingleClient(QObject *parent) : QObject{parent}
{
    connect(EventDispatcher::instance()->client(), &ClientSignals::removed, this, [this](QList<int> removedIds) {
        if (removedIds.contains(id()))
        {
            resetId();
        }
    });
    connect(EventDispatcher::instance()->client(), &ClientSignals::updated, this, [this](ClientDTO dto) {
        if (dto.id() == id())
        {

            if (m_id != dto.id())
            {
                m_id = dto.id();
                emit idChanged();
            }
            if (m_uuid != dto.uuid())
            {
                m_uuid = dto.uuid();
                emit uuidChanged();
            }
            if (m_creationDate != dto.creationDate())
            {
                m_creationDate = dto.creationDate();
                emit creationDateChanged();
            }
            if (m_updateDate != dto.updateDate())
            {
                m_updateDate = dto.updateDate();
                emit updateDateChanged();
            }
        }
    });
}

int SingleClient::id() const
{
    return m_id;
}

void SingleClient::setId(int newId)
{
    if (m_id == newId)
        return;
    m_id = newId;
    emit idChanged();

    // clear
    if (m_id == 0)
    {

        m_uuid = QUuid();
        emit uuidChanged();

        m_creationDate = QDateTime();
        emit creationDateChanged();

        m_updateDate = QDateTime();
        emit updateDateChanged();
    }

    // set
    else
    {
        Client::ClientController::instance()->get(m_id).then(
            [this](const Simple::Contracts::DTO::Client::ClientDTO &client) {
                m_uuid = client.uuid();
                emit uuidChanged();

                m_creationDate = client.creationDate();
                emit creationDateChanged();

                m_updateDate = client.updateDate();
                emit updateDateChanged();
            });
    }
}

void SingleClient::resetId()
{
    setId(0);
}

QString SingleClient::name() const
{
    return m_name;
}

void SingleClient::setName(const QString &newName)
{
    if (m_name == newName)
        return;
    m_name = newName;

    UpdateClientDTO dto;
    dto.setId(id());
    dto.setName(newName);
    Client::ClientController::instance()->update(dto);

    emit nameChanged();
}