// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "simple_example_controller_export.h"

#include "client/client_with_details_dto.h"

#include "client/client_dto.h"

#include "client/client_relation_dto.h"

#include <QObject>

namespace Simple::Controller
{

using namespace Simple::Contracts::DTO::Client;

class SIMPLE_EXAMPLE_CONTROLLER_EXPORT ClientSignals : public QObject
{
    Q_OBJECT
  public:
    explicit ClientSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  Q_SIGNALS:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool isActive);
    void created(ClientDTO dto);
    void updated(ClientDTO dto);
    void allRelationsInvalidated(int id);
    void getReplied(ClientDTO dto);
    void getWithDetailsReplied(ClientWithDetailsDTO dto);
    void getAllReplied(QList<ClientDTO> dtoList);

    void relationInserted(ClientRelationDTO dto);
    void relationRemoved(ClientRelationDTO dto);
};
} // namespace Simple::Controller