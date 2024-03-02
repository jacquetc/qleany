// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "client/client_dto.h"
#include "simple_example_application_client_export.h"

#include "repository/interface_client_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Simple::Contracts::DTO::Client;
using namespace Simple::Contracts::Repository;

namespace Simple::Application::Features::Client::Queries
{
class SIMPLE_EXAMPLE_APPLICATION_CLIENT_EXPORT GetAllClientQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetAllClientQueryHandler(InterfaceClientRepository *repository);
    Result<QList<ClientDTO>> handle(QPromise<Result<void>> &progressPromise);

  private:
    InterfaceClientRepository *m_repository;
    Result<QList<ClientDTO>> handleImpl(QPromise<Result<void>> &progressPromise);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Client::Queries