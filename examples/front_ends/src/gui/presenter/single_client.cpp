// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "single_client.h"
#include "client/client_controller.h"
#include "event_dispatcher.h"

using namespace FrontEnds::Controller;
using namespace FrontEnds::Presenter;

SingleClient::SingleClient(QObject *parent)
    : QObject{parent}
{
    connect(EventDispatcher::instance()->client(), &ClientSignals::removed, this, [this](QList<int> removedIds) {
        if (removedIds.contains(id())) {
            resetId();
        }
    });
    connect(EventDispatcher::instance()->client(), &ClientSignals::updated, this, [this](ClientDTO dto) {
        if (dto.id() == id()) {
            if (m_id != dto.id()) {
                m_id = dto.id();
                Q_EMIT idChanged();
            }
            if (m_uuid != dto.uuid()) {
                m_uuid = dto.uuid();
                Q_EMIT uuidChanged();
            }
            if (m_creationDate != dto.creationDate()) {
                m_creationDate = dto.creationDate();
                Q_EMIT creationDateChanged();
            }
            if (m_updateDate != dto.updateDate()) {
                m_updateDate = dto.updateDate();
                Q_EMIT updateDateChanged();
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
    Q_EMIT idChanged();

    // clear
    if (m_id == 0) {
        m_uuid = QUuid{};
        Q_EMIT uuidChanged();

        m_creationDate = QDateTime{};
        Q_EMIT creationDateChanged();

        m_updateDate = QDateTime{};
        Q_EMIT updateDateChanged();

    }

    // set
    else {
        Client::ClientController::instance()->get(m_id).then([this](const FrontEnds::Contracts::DTO::Client::ClientDTO &client) {
            if (client.isInvalid()) {
                qCritical() << Q_FUNC_INFO << "Invalid clientId";
                return;
            }

            m_uuid = client.uuid();
            Q_EMIT uuidChanged();

            m_creationDate = client.creationDate();
            Q_EMIT creationDateChanged();

            m_updateDate = client.updateDate();
            Q_EMIT updateDateChanged();
        });
    }
}

void SingleClient::resetId()
{
    setId(0);
}

QUuid SingleClient::uuid() const
{
    return m_uuid;
}

void SingleClient::setUuid(const QUuid &newUuid)
{
    if (m_uuid == newUuid)
        return;

    UpdateClientDTO dto;
    dto.setId(id());
    dto.setUuid(newUuid);
    Client::ClientController::instance()->update(dto).then([this](const FrontEnds::Contracts::DTO::Client::ClientDTO &client) {
        if (client.isInvalid()) {
            qCritical() << Q_FUNC_INFO << "Invalid clientId";
            return;
        }
        m_uuid = client.uuid();
        Q_EMIT uuidChanged();
    });
}

QDateTime SingleClient::creationDate() const
{
    return m_creationDate;
}

void SingleClient::setCreationDate(const QDateTime &newCreationDate)
{
    if (m_creationDate == newCreationDate)
        return;

    UpdateClientDTO dto;
    dto.setId(id());
    dto.setCreationDate(newCreationDate);
    Client::ClientController::instance()->update(dto).then([this](const FrontEnds::Contracts::DTO::Client::ClientDTO &client) {
        if (client.isInvalid()) {
            qCritical() << Q_FUNC_INFO << "Invalid clientId";
            return;
        }
        m_creationDate = client.creationDate();
        Q_EMIT creationDateChanged();
    });
}

QDateTime SingleClient::updateDate() const
{
    return m_updateDate;
}

void SingleClient::setUpdateDate(const QDateTime &newUpdateDate)
{
    if (m_updateDate == newUpdateDate)
        return;

    UpdateClientDTO dto;
    dto.setId(id());
    dto.setUpdateDate(newUpdateDate);
    Client::ClientController::instance()->update(dto).then([this](const FrontEnds::Contracts::DTO::Client::ClientDTO &client) {
        if (client.isInvalid()) {
            qCritical() << Q_FUNC_INFO << "Invalid clientId";
            return;
        }
        m_updateDate = client.updateDate();
        Q_EMIT updateDateChanged();
    });
}
