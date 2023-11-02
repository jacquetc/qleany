// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_brand_export.h"
#include "brand/brand_dto.h"
#include "brand/commands/remove_brand_command.h"

#include "qleany/common/result.h"
#include "repository/interface_brand_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Simple::Contracts::DTO::Brand;
using namespace Simple::Contracts::Repository;
using namespace Simple::Contracts::CQRS::Brand::Commands;

namespace Simple::Application::Features::Brand::Commands
{
class SIMPLEEXAMPLE_APPLICATION_BRAND_EXPORT RemoveBrandCommandHandler : public QObject
{
    Q_OBJECT
  public:
    RemoveBrandCommandHandler(InterfaceBrandRepository *repository);
    Result<int> handle(QPromise<Result<void>> &progressPromise, const RemoveBrandCommand &request);
    Result<int> restore();

  signals:
    // repositories handle remove signals
    // void brandRemoved(int id);

  private:
    InterfaceBrandRepository *m_repository;
    Result<int> handleImpl(QPromise<Result<void>> &progressPromise, const RemoveBrandCommand &request);
    Result<int> restoreImpl();
    Simple::Domain::Brand m_oldState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Simple::Application::Features::Brand::Commands