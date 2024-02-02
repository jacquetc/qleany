// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "interactor_export.h"

#include "client/client_with_details_dto.h"

#include "client/client_dto.h"

#include "client/client_relation_dto.h"

#include <QObject>

namespace Simple::Interactor
{

using namespace Simple::Contracts::DTO::Client;

class SIMPLEEXAMPLE_INTERACTOR_EXPORT ClientSignals : public QObject
{
    Q_OBJECT
  public:
    explicit ClientSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool isActive);
    void getReplied(ClientDTO dto);
    void getWithDetailsReplied(ClientWithDetailsDTO dto);
    void getAllReplied(QList<ClientDTO> dtoList);
    void created(ClientDTO dto);
    void updated(Contracts::DTO::Client::ClientDTO dto);
    void allRelationsInvalidated(int id);

    void relationInserted(ClientRelationDTO dto);
    void relationRemoved(ClientRelationDTO dto);
};
} // namespace Simple::Interactor